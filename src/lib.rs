use dotenv::dotenv;
use openai_flows::chat_completion;
use slack_flows::{listen_to_channel, send_message_to_channel};
use std::env;
#[no_mangle]
pub fn run() {
    dotenv().ok();
    let TEAM_NAME: String = match env::var("TEAM_NAME") {
        Err(_) => "ik8".to_string(),
        Ok(name) => name,
    };

    let CHANNEL_NAME: String = match env::var("TEAM_NAME") {
        Err(_) => "general".to_string(),
        Ok(name) => name,
    };
    let OPENPI_KEY_NAME: String = match env::var("TEAM_NAME") {
        Err(_) => "jaykchen".to_string(),
        Ok(name) => name,
    };
    listen_to_channel(&TEAM_NAME, &CHANNEL_NAME, |sm| {
        let CHAT_ID: String = match env::var("TEAM_NAME") {
            Err(_) => "jaykchen".to_string(),
            Ok(name) => name,
        };

        let c = chat_completion("Agent", &CHAT_ID, &sm.text);
        if let Some(c) = c {
            if c.new_conversation {
                send_message_to_channel(
                    &OPENPI_KEY_NAME,
                    &CHANNEL_NAME,
                    "Let's start a new conversation!".to_string(),
                );
            }
            send_message_to_channel(&TEAM_NAME, &CHANNEL_NAME, c.choice);
        }
    });
}