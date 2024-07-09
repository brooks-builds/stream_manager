use change_alacritty_font::get_current_font;
use change_helix_theme::current_helix_theme;
use dotenvy::dotenv;
use eyre::Context;
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
    let helix_theme = current_helix_theme(&config.helix_config_path)
        .context("loading helix theme")
        .unwrap();
    let font = get_current_font(config.alacritty_font_config_path.clone())
        .context("getting font")
        .unwrap();

    // create anathema runtime builder
    // create anathema emitter

    tokio::spawn(async {
        run(twitch_events_listener_config, sender, user_token)
            .await
            .unwrap();
    });

    tokio::spawn(async move {
        // send anathema runtime builder into the function
        frontend::run(frontend_receiver, helix_theme, font).unwrap();
    });

    // send anathema emitter
    stream_manager::run(receiver, config, frontend_sender)
        .await
        .unwrap();
}
