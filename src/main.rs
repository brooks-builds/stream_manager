use std::sync::mpsc::{self};

use dotenvy::dotenv;
use stream_manager::config::Config;
use twitch_events_listener::{config::Config as TwitchEventsListenerConfig, run};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let twitch_events_listener_config = TwitchEventsListenerConfig::new_from_env().unwrap();
    let config = Config::new("/Users/brookzerker/.config/helix/config.toml");
    let (sender, receiver) = mpsc::channel();

    tokio::spawn(async {
        run(twitch_events_listener_config, sender).await.unwrap();
    });

    stream_manager::run(receiver, config).unwrap();
}
