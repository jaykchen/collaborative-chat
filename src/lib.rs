use dotenv::dotenv;
use openai_flows::chat_completion;
use slack_flows::{listen_to_channel, send_message_to_channel};
use std::env;
#[no_mangle]
pub fn run() {
    // dotenv().ok();
    // let TEAM_NAME: String = match env::var("TEAM_NAME") {
    //     Err(_) => "ik8".to_string(),
    //     Ok(name) => name,
    // };

    // let CHANNEL_NAME: String = match env::var("CHANNEL_NAME") {
    //     Err(_) => "general".to_string(),
    //     Ok(name) => name,
    // };
    // let OPENPI_KEY_NAME: String = match env::var("OPENPI_KEY_NAME") {
    //     Err(_) => "jaykchen".to_string(),
    //     Ok(name) => name,
    // };
    listen_to_channel("ik8", "general", |sm| {
        // let CHAT_ID: String = match env::var("CHAT_ID") {
        //     Err(_) => "secondstate-collaborative_chat".to_string(),
        //     Ok(id) => id,
        // };

        let c = chat_completion("jaykchen", "secondstate-collaborative_chat", &sm.text);
        if let Some(c) = c {
            if c.new_conversation {
                send_message_to_channel(
                    "ik8",
                    "general",
                    "Let's start a new conversation!".to_string(),
                );
            }
            send_message_to_channel("ik8", "general", c.choice);
        }
    });
}
