use std::fmt;
use crate::geo_coder::GeoCoords;
use telegram_bot::*;
use crate::command_handler::Cmdr;
use std::collections::HashMap;
// use crate::bot::CommandKind::AddMap;
// use crate::bot::AddCommandStep::TopLeft;
// use telegram_bot::{Api, Message, MessageKind};

// struct AddMapContext

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

// #[derive(Default)]
struct AddMapCommand {
    step: AddCommandStep,
    pub top_left: Option<GeoCoords>,
    pub bottom_right: Option<GeoCoords>,
    pub image: Option<String>,
    api: Api,
    token: String,
    // api: Api
}

// message: &Message, api: &Api
impl AddMapCommand {
    pub fn new(api: &Api, token: &String) -> Self {
        return Self {
            token: token.clone(),
            api: api.clone(),
            top_left: None,
            bottom_right: None,
            image: None,
            step: AddCommandStep::default(),
            // ..Self::default()
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
                            self.step = AddCommandStep::Image;
                            self.api.send(message.chat.text("#3. Прикрепите изображение карты").parse_mode(ParseMode::Markdown)).await?;
                        }
                    }
                }
            }
            AddCommandStep::Image => {
                if let MessageKind::Photo {ref data, .. } = message.kind {
                    let file_url =data[0].get_file();
                    let resp = self.api.send(file_url).await?;
                    let full_url = resp.get_url(self.token.as_str()).unwrap();
                    if let Some(file_url) = resp.file_path {
                        println!("Got right file {}", full_url);
                    }
                    // file_url.url
                    // let file = reqwest.get()reqwest.data[0].get_file(;

                    // self.api.send(message.chat.text(format!("{}", )).parse_mode(ParseMode::Markdown))

                }

            }

            // _ => {}
        }
        return Ok(());
    }
}

enum CommandKind {
    AddMap(AddMapCommand),
    // Help,
}


// struct Context {
//     current_command: Option<CommandKind>,
// }

// impl Context {
//     fn new() -> Self {
//         return Self {
//             current_command: None,
//         };
//     }
// }

// //
// fn command_add_map(message: Message, context: &mut Context) {
//     context.current_command = Some(Command::AddMap);
// }

// &Context
// type Call<T> = ;
// type Comands<'r, T> = HashMap<String, Box<dyn for<'a> FnOnce(&'a mut T, &Message)>>;

pub struct Bot<'a> {
    // commands: Comands<'a, Self>,
    token: String,
    current_command: Option<CommandKind>,
    // context: Context,
    api: &'a Api,
}


impl<'a> Bot<'a> {
    pub fn new(api: &'a Api, token: String) -> Self {

        // let response = reqwest::get(target).await?;
        // let content =  response.text().await?.as_bytes();
        // let aaa = String::from("sdf");
        // let bbb = aaa;
        // let ccc = aaa;
        // let mut commands: Comands<Self> = HashMap::new();
        // commands.insert(String::from("+++"), Box::new(command_add_map));
        // commands.insert(String::from("+++ddd"), Box::new(Self::command_bbbb));
        // let mut commands = Cmdr::new();
        // commands.add("new_map", command_add_map);
        Self {
            current_command: None,
            // commands: commands,
            token,
            api,
            // context: Context::new(),
        }
    }

    async fn handle_add_map(&mut self, message: &Message) {
        // match self.current_command.as_mut() {
        //     Some(AddMap(command))  => {
        //         if command.top_left.is_none() {
        //             self.api.send(message.text_reply())
        //         } else if command.bottom_right.is_none() {
        //
        //         } else if command.image.is_none() {
        //
        //         }
        //
        //
        //
        //         // println!("====3");
        //         command.top_left = Some(GeoCoords(10.0, 10.0));
        //     }
        //     _ => {
        //         self.current_command = Some(AddMap(AddMapCommand::default()));
        //     }
        // }
        // self.current_command = CommandKind::AddMap::new();
        // if self.context.current_command.is_none() {
        //
        //     // self.context.top_left = None;
        //     // self.context.bottom_right = None;
        //     // self.context.image = None;
        //     // self.context.current_command = Some(CommandKind::AddMap { top_left: None, bottom_right: None, image: Some("".to_string()) });
        //     // match self.context.current_command.as_mut() {
        //     //     Some(CommandKind::AddMap { ref mut top_left, .. }) => {
        //     //         let coords = top_left.take();
        //     //         // top_left.as_mut() = GeoCoords(10., 10.);
        //     //         // top_left. = Some(GeoCoords(10., 10.));
        //     //     }
        //     //     _ => {}
        //     //     // daa; // self.context.current_command.unwrap().top;
        //     // }
        //     // let bbb = self.context.current_command.unwrap();
        // } else {}
        // let mut reply_message = "+++";
        // let reply_message = if self.context.top_left.is_none() {
        //     "Введите гео координаты в формате: `12.3456,12.3456`"
        // } else {
        //     "+++"
        // };
        // self.api.send(message.chat.text(reply_message).parse_mode(ParseMode::Markdown)).await.unwrap();
        // message.reply_to_message();
        // self.api.send();
        // println!("command_bbbb");
        // context.current_command = Some(Command::AddMap);

        // context.top_left = Some(GeoCoords(10., 10.));
    }


    pub async fn handle_message(&mut self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        if let MessageKind::Text { ref data, .. } = message.kind {
            if data.starts_with("/") {
                // if data.starts_with("/") {
                match data.as_str() {
                    "/add_map" => {
                        let mut add_map_command = AddMapCommand::new(&self.api, &self.token);
                        add_map_command.handle_command(&message).await?;
                        self.current_command = Some(CommandKind::AddMap(add_map_command));
                        // self.current_command
                        // let mut ddd = CommandKind::AddMap(AddMapCommand::default());
                        // match ddd {
                        //     CommandKind::AddMap(ref mut  command) => {
                        //         command.handle_command();
                        //     }
                        //     // _ => None
                        // }
                        // ddd
                        // self.current_command.unwrap()
                        // self.current_command.unwrap()
                        // self.handle_add_map(&message).await
                    }
                    _ => println!("")
                }
                // match self.commands.get_mut("+++") {
                //     // &self.context
                //     Some(fff) => {
                //         let xxx = fff;
                //         let bbb= xxx.as_ref();
                //         (bbb)(self, &message);
                //     },
                //     _ => println!("dnf")
                // };
                // println!("Got command");
                // handle_command(data);
                // return Ok(());
                // }
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                // self.api.send(message.text_reply(format!(
                //     "Hi, {}! You joust wrote '{}'",
                //     &message.from.first_name, data
                // )))
                //     .await?;
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

        // else if let MessageKind::Photo { ref data, .. } = message.kind {
        //     let files = message.get_files();
        //     let ref file = files.unwrap()[0];
        //     let resp = self.api.send(file).await?;
        //     // resp.file_path
        //     // file.get();
        //     // file.
        //     let photo_size = data.first().unwrap();
        //     println!("{}", photo_size.file_id);
        //     let response = reqwest::get(&self.token).await.unwrap();
        //     // let mut dest = {
        //     let fname = response
        //         .url()
        //         .path_segments()
        //         .and_then(|segments| segments.last())
        //         .and_then(|name| if name.is_empty() { None } else { Some(name) })
        //         .unwrap_or("tmp.bin");
        //
        //     // println!("file to download: '{}'", fname);
        //     // let fname = tmp_dir.path().join(fname);
        //     // println!("will be located under: '{:?}'", fname);
        //     // File::create(fname)?
        //     // };
        //     let content = response.text().await.unwrap();
        //     // generate_page(content.as_bytes());
        //     let file_content = file.serialize().unwrap();
        //     match file_content.body {
        //         Body::Multipart(body) => {
        //             println!("!!!")
        //         }
        //         Body::Json(json) => {
        //             println!("{} {}JSOOOOON", json, resp.get_url(&self.token.as_str()).unwrap());
        //             // reqwest::get(target).await?;
        //             // json
        //         }
        //         _ => {}
        //     }
        //     // { }let raw = file_content.body;
        //     // print!("{} ggg", file..to_ascii_lowercase());
        //     // // println!("{}", raw.as_bytes());
        //     // let ff = match files {
        //     //
        //     //     Some(quotient) => {
        //     //         files[0]
        //     //         // println!("{} / {} = {}", dividend, divisor, quotient)
        //     //     },
        //     //     _ => println!("{} / {} failed!", dividend, divisor)
        //     // };
        //
        //     // let _image_data: Option<Vec<GetFile>> = Some(data.into_iter().map(|f| f.get_file()).collect());
        //     // _image_data
        //
        //     // println!("Photo message")
        // }
        ;
        Ok(())
    }

    fn handle_command(&self) {}
}

struct BotCommandError {}

fn command_start() -> Result<(), BotCommandError> {
    Ok(())
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
        let bt = Bot::new(&api, token.clone());
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


// fn command_start() {
//
// }
//
// #[derive(Debug, Clone)]
// struct BasicError;
//
// // Generation of an error is completely separate from how it is displayed.
// // There's no need to be concerned about cluttering complex logic with the display style.
// //
// // Note that we don't store any extra info about the errors. This means we can't state
// // which string failed to parse without modifying our types to carry that information.
// impl fmt::Display for BasicError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "invalid first item to double")
//     }
// }
//



