use std::fmt;
use crate::geo_coder::GeoCoords;
use telegram_bot::*;
// use telegram_bot::{Api, Message, MessageKind};


struct Context<'a> {
    top_left: Option<GeoCoords>,
    bottom_right: Option<GeoCoords>,
    image: Option<&'a [u8]>,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        return Self {
            top_left: None,
            bottom_right: None,
            image: None,
        };
    }
}

pub struct Bot<'a> {
    token: String,
    context: Context<'a>,
    api: &'a Api,
}


impl<'a> Bot<'a> {
    pub fn new(api: &'a Api, token: String) -> Self {
        Self {
            token,
            api,
            context: Context::new(),
        }
    }

    pub async fn handle_message(&self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        if let MessageKind::Text { ref data, .. } = message.kind {
            if data.starts_with("/") {
                println!("Got command");
                // handle_command(data);
                return Ok(());
            }


            // Print received text message to stdout.
            println!("<{}>: {}", &message.from.first_name, data);

            // Answer message with "Hi".
            self.api.send(message.text_reply(format!(
                "Hi, {}! You joust wrote '{}'",
                &message.from.first_name, data
            )))
                .await?;
        } else if let MessageKind::Photo { ref data, .. } = message.kind {
            let files = message.get_files();
            let ref file = files.unwrap()[0];
            let resp = self.api.send(file).await?;
            // resp.file_path
            // file.get();
            // file.
            let photo_size = data.first().unwrap();
            println!("{}", photo_size.file_id);
            let response = reqwest::get(&self.token).await.unwrap();
            // let mut dest = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            // println!("file to download: '{}'", fname);
            // let fname = tmp_dir.path().join(fname);
            // println!("will be located under: '{:?}'", fname);
            // File::create(fname)?
            // };
            let content = response.text().await.unwrap();
            // generate_page(content.as_bytes());
            let file_content = file.serialize().unwrap();
            match file_content.body {
                Body::Multipart(body) => {
                    println!("!!!")
                }
                Body::Json(json) => {
                    println!("{} {}JSOOOOON", json, resp.get_url(&self.token.as_str()).unwrap());
                    // reqwest::get(target).await?;
                    // json
                }
                _ => {}
            }
            // { }let raw = file_content.body;
            // print!("{} ggg", file..to_ascii_lowercase());
            // // println!("{}", raw.as_bytes());
            // let ff = match files {
            //
            //     Some(quotient) => {
            //         files[0]
            //         // println!("{} / {} = {}", dividend, divisor, quotient)
            //     },
            //     _ => println!("{} / {} failed!", dividend, divisor)
            // };

            // let _image_data: Option<Vec<GetFile>> = Some(data.into_iter().map(|f| f.get_file()).collect());
            // _image_data

            // println!("Photo message")
        };
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



