use std::fmt;
use std::fs;
use url::{Url as UrlLib, ParseError};

use crate::geo_coder::GeoCoords;
use telegram_bot::*;
use std::collections::HashMap;
use futures::TryFutureExt;
use crate::tracker::publish_map;
use config::Config;
use std::path::Path;

enum AddCommandStep {
    Initial,
    TopLeft,
    BottomRight,
    Image,
}

impl Default for AddCommandStep {
    fn default() -> Self {
        AddCommandStep::Initial
    }
}

struct AddMapCommand {
    step: AddCommandStep,
    pub top_left: Option<GeoCoords>,
    pub bottom_right: Option<GeoCoords>,
    pub image: Option<String>,
    api: Api,
    token: String,
}

impl AddMapCommand {
    pub fn new(api: &Api, token: String) -> Self {
        return Self {
            token: token,
            api: api.clone(),
            top_left: None,
            bottom_right: None,
            image: None,
            step: AddCommandStep::default(),
        };
    }
    pub async fn handle_command(&mut self, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        match self.step {
            AddCommandStep::Initial => {
                self.api.send(message.chat.text("#1. Введите координаты `верхнего левого угла` картинки  `12.3456, 12.3456`").parse_mode(ParseMode::Markdown)).await.unwrap();
                self.step = AddCommandStep::TopLeft;
            }
            AddCommandStep::TopLeft => {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    let coords = GeoCoords::from_str(data);
                    match coords {
                        Err(error) => {
                            self.api.send(
                                message.chat.text("#1. Ошибка в координатах. Формат: `12.3456, 12.3456`"
                                ).parse_mode(ParseMode::Markdown)
                            ).await?;
                        }
                        Ok(coords) => {
                            self.top_left = Some(coords);
                            self.step = AddCommandStep::BottomRight;
                            self.api.send(message.chat.text("#2. Введите координаты `нижнего правого угла` картинки  `12.3456, 12.3456`").parse_mode(ParseMode::Markdown)).await?;
                        }
                    }
                } else {
                    self.api.send(
                        message.chat.text("#1. Введите координаты `верхнего левого угла` картинки  `12.3456, 12.3456`"
                        ).parse_mode(ParseMode::Markdown)
                    ).await?;
                }
            }
            AddCommandStep::BottomRight => {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    let coords = GeoCoords::from_str(data);
                    match coords {
                        Err(error) => {
                            self.api.send(
                                message.chat.text("#2. Ошибка в координатах. Формат: `12.3456, 12.3456`"
                                ).parse_mode(ParseMode::Markdown)
                            ).await?;
                        }
                        Ok(coords) => {
                            self.bottom_right = Some(coords);
                            self.step = AddCommandStep::Image;
                            self.api.send(message.chat.text("#3. Прикрепите изображение карты").parse_mode(ParseMode::Markdown)).await?;
                        }
                    }
                }
            }
            AddCommandStep::Image => {
                let mut file_url: Option<GetFile> = None;
                if let MessageKind::Photo { ref data, .. } = message.kind {
                    file_url = Some(data[0].get_file());
                } else if let MessageKind::Document { ref data, .. } = message.kind {
                    file_url = Some(data.get_file());
                    // data.mime_type
                } else {
                    self.api.send(message.chat.text("#3. Не могу распознать изображение")).await?;
                    return Ok(());
                }

                let file = self.api.send(file_url.unwrap()).await?;
                let full_url = file.get_url(self.token.as_str()).unwrap();
                if let Some(file_url) = &file.file_path {
                    let resp = reqwest::get(&full_url).await?;
                    let bytes = resp.bytes().await?;
                    let dest_url = publish_map(
                        &self.top_left.as_ref().unwrap(),
                        &self.bottom_right.as_ref().unwrap(),
                        &bytes.to_vec(),
                        file_url,
                    );

                    let message_text = format!("{}", dest_url.as_str()).replace("`", r"\`").replace(")", r"\)");
                    self.api.send(message.chat.text(message_text).parse_mode(ParseMode::Html)).await?;
                }
            }
        }
        return Ok(());
    }
}


struct ListCommand {
    settings: Config,
    api: Api,
}


fn list_html_files(storage_path: &str, server_url: &str) -> String {
    let dir_path = storage_path;
    let paths = fs::read_dir(dir_path).unwrap();
    let dest_url = UrlLib::parse(server_url).unwrap();

    let urls = paths.
        filter(|f| {
            let dir_entry = f.as_ref().unwrap();
            let path = dir_entry.path();
            let metadata = fs::metadata(&path).unwrap();
            metadata.is_file() && path.extension().unwrap() == "html"
        }).flatten().
        map(|f| {
            let url = dest_url.join(f.file_name().to_str().unwrap()).unwrap().to_string();
            format!("{}", url.as_str()).replace("`", r"\`").replace(")", r"\)")
        }).collect::<Vec<_>>();
    urls.join("\n")
}

impl ListCommand {
    pub fn new(api: &Api, settings: Config) -> Self {
        return Self {
            settings,
            api: api.clone(),
        };
    }
    pub async fn handle_command(&mut self, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let files_as_text = list_html_files(
            &self.settings.get_str("map_storage_path").unwrap(),
            &self.settings.get_str("server_url").unwrap(),
        );
        let mut message_text: String = String::from("");
        if files_as_text.is_empty() {
            message_text = String::from("Нет карт");
        } else {
            message_text = files_as_text;
        }

        self.api.send(message.chat.text(message_text).parse_mode(ParseMode::Html)).await?;

        Ok(())
    }
}

enum CommandKind {
    AddMap(AddMapCommand),
    List(ListCommand),
}


pub struct Bot<'a> {
    settings: Config,
    current_command: Option<CommandKind>,
    api: &'a Api,
}


impl<'a> Bot<'a> {
    pub fn new(api: &'a Api, settings: Config) -> Self {
        Self {
            current_command: None,
            settings,
            api,
        }
    }

    pub async fn handle_message(&mut self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        if let MessageKind::Text { ref data, .. } = message.kind {
            if data.starts_with("/") {
                match data.as_str() {
                    "/add_map" => {
                        let mut add_map_command = AddMapCommand::new(&self.api, self.settings.get_str("bot_token").unwrap());
                        add_map_command.handle_command(&message).await?;
                        self.current_command = Some(CommandKind::AddMap(add_map_command));
                    }
                    "/list" => {
                        let mut list_command = ListCommand::new(&self.api, self.settings.clone());
                        list_command.handle_command(&message).await?;
                        self.current_command = Some(CommandKind::List(list_command));
                    }
                    _ => {}
                }

                return Ok(());
            }
        }
        if !self.current_command.is_none() {
            match self.current_command.as_mut() {
                Some(CommandKind::AddMap(command)) => {
                    command.handle_command(&message).await?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn handle_command(&self) {}
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn bot_commands() {
        let token = "".to_string();
        let api = Api::new(token.clone());
        // assert_eq!(2, 3);
        // let bt = Bot::new(&api, token.clone());
    }

    #[test]
    fn vvvv() {
        // print!("++++");
        assert_eq!(list_html_files("/tmp/track_mapper", "http://localhost:8000"), "+++");
    }
}
