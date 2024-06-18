use dotenvy::dotenv;
use std::sync::mpsc::{self};
use stream_manager::config::Config;
use twitch_events_listener::{config::Config as TwitchEventsListenerConfig, run};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let twitch_events_listener_config = TwitchEventsListenerConfig::new_from_env().unwrap();
    let config = Config::new(
        "/Users/brookzerker/.config/helix/config.toml",
        "/Users/brookzerker/.config/alacritty/font.toml",
    );
    let (sender, receiver) = mpsc::channel();

    tokio::spawn(async {
        frontend::run().unwrap();
    });

    tokio::spawn(async {
        run(twitch_events_listener_config, sender).await.unwrap();
    });

    stream_manager::run(receiver, config).await.unwrap();
}
