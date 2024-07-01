use dotenvy::dotenv;
use std::sync::mpsc::{self};
use stream_manager::config::Config;
use twitch_events_listener::{config::Config as TwitchEventsListenerConfig, get_code, run};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let twitch_events_listener_config = TwitchEventsListenerConfig::new_from_env().unwrap();
    let config = Config::new(
        "/Users/brookzerker/.config/helix/config.toml",
        "/Users/brookzerker/.config/alacritty/font.toml",
    );
    let (sender, receiver) = mpsc::channel();
    let user_token = get_code(&twitch_events_listener_config).await.unwrap();
    let (frontend_sender, frontend_receiver) =
        tokio::sync::mpsc::channel::<frontend::events::Events>(10);

    tokio::spawn(async {
        run(twitch_events_listener_config, sender, user_token)
            .await
            .unwrap();
    });

    tokio::spawn(async {
        frontend::run(frontend_receiver).unwrap();
    });

    stream_manager::run(receiver, config, frontend_sender)
        .await
        .unwrap();
}
