pub mod config;

use change_alacritty_font::change_alacritty_font;
use config::Config;
use eyre::{Context, Result};
use std::{sync::mpsc::Receiver, time::Duration};
use tokio::time::sleep;
use twitch_events_listener::stream_event::StreamEvent;

pub async fn run(stream_events: Receiver<StreamEvent>, config: Config) -> Result<()> {
    loop {
        let config = config.clone();
        let stream_event = stream_events
            .recv()
            .context("getting stream event")
            .unwrap();

        tokio::spawn(async move {
            match stream_event {
                StreamEvent::ChangeHelixTheme { username, theme } => {
                    println!("{username} changed the Helix theme to {theme}");

                    change_helix_theme::change_helix_theme(&config.helix_config_path, &theme)
                        .unwrap()
                }
                StreamEvent::ChangeFont { username, font } => {
                    println!("{username} changing the font to {font}");
                    let path = config.alacritty_font_config_path.clone();

                    let previous_font = change_alacritty_font(&path, &font, &username, false)
                        .unwrap_or("Iosevka".to_owned());

                    sleep(Duration::from_secs(60 * 10)).await;

                    change_alacritty_font(&path, &previous_font, "brookzerker", true).unwrap();
                }
                StreamEvent::Unknown => eprintln!("got an unknown stream event"),
            }
        })
        .await
        .context("running everything")
        .unwrap();
    }
}
