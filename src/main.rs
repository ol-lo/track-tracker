mod page_builder;
mod geo_coder;
mod bot;
mod command_handler;

use std::env;

use futures::StreamExt;
use telegram_bot::*;
use crate::page_builder::generate_page;
use crate::bot::Bot;


enum State {
    Start
}

struct FSM {
    state: State,
}

impl FSM {
    pub fn new() -> Self {
        FSM {
            state: State::Start
        }
    }

    pub fn handle_command(&self, command: &str) {

    }

}
// static mut STATE: State = State::Start;

// fn add_map() {
//     STATE = State::Start;
// }

// "add_map" => Some(AddMap),
// fn handle_command(command_str: &str) {
//     match command_str {
//         "add_map" => add_map(),
//
//         // "ggg" => Some(X::Xxx),
//         _ => None
//     };
//     // println!("{:?}", command);
// }
// Result<>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(&token);
    // let fsm = FSM::new();
    // fsm.xx();
    let bot = Bot::new(&api, token);
    // Message::try_from()
    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            bot.handle_message(message).await?;
            // if let MessageKind::Text { ref data, .. } = message.kind {
            //     if !data.starts_with("/") {
            //         // handle_command(data);
            //     }
            //
            //
            //     // Print received text message to stdout.
            //     println!("<{}>: {}", &message.from.first_name, data);
            //
            //     // Answer message with "Hi".
            //     api.send(message.text_reply(format!(
            //         "Hi, {}! You joust wrote '{}'",
            //         &message.from.first_name, data
            //     )))
            //         .await?;
            // } else if let MessageKind::Photo { ref data, .. } = message.kind {
            //     let files = message.get_files();
            //     let ref file = files.unwrap()[0];
            //     let resp = api.send(file).await?;
            //     // resp.file_path
            //     // file.get();
            //     // file.
            //     let photo_size = data.first().unwrap();
            //     println!("{}", photo_size.file_id);
            //     let response = reqwest::get(&token).await.unwrap();
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
            //     generate_page(content.as_bytes());
            //     let file_content = file.serialize().unwrap();
            //     match file_content.body {
            //         Body::Multipart(body) => {
            //             println!("!!!")
            //         }
            //         Body::Json(json) => {
            //             println!("{} {}JSOOOOON", json, resp.get_url(&token.as_str()).unwrap());
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
        }
    }
    Ok(())
}
