mod geo_coder;
mod bot;
mod tracker;

use std::env;

use futures::StreamExt;
use telegram_bot::*;
use crate::bot::Bot;
use std::collections::HashMap;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();

    let token = settings.get_str("bot_token").unwrap();
    let api = Api::new(&token);
    let mut bot = Bot::new(&api, settings.clone());
    let mut stream = api.stream();


    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            bot.handle_message(message).await?;
        }
    }
    Ok(())
}
