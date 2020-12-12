use std::fmt;
use crate::geo_coder::GeoCoords;
use telegram_bot::*;
use crate::command_handler::Cmdr;
use std::collections::HashMap;
use futures::TryFutureExt;
use crate::tracker::publish_map;
use config::Config;

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
                if let MessageKind::Photo { ref data, .. } = message.kind {
                    let file_url = data[0].get_file();

                    let file = self.api.send(file_url).await?;
                    let full_url = file.get_url(self.token.as_str()).unwrap();
                    if let Some(file_url) = &file.file_path {
                        let resp = reqwest::get(&full_url).await?;
                        let bytes = resp.bytes().await?;
                        let dest_url = publish_map(
                            &self.top_left.as_ref().unwrap(),
                            &self.bottom_right.as_ref().unwrap(),
                            &bytes.to_vec(),
                            file_url
                        );

                        let message_text = format!("[{}]({})", dest_url.as_str(), dest_url.as_str()).replace("`", r"\`").replace(")", r"\)");
                        self.api.send(message.chat.text(message_text).parse_mode(ParseMode::Markdown)).await?;
                    }
                }
            }
        }
        return Ok(());
    }
}

enum CommandKind {
    AddMap(AddMapCommand),
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
        fn xxx(aaa: String) -> String {
            aaa
        }

        let mut ppp: String = String::from("++++");
        ppp = xxx(ppp);

        println!("{}", ppp)
    }
}
