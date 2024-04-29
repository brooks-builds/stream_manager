pub mod config;

use config::Config;
use eyre::{Context, Result};
use std::sync::mpsc::Receiver;
use twitch_events_listener::stream_event::StreamEvent;

pub fn run(stream_events: Receiver<StreamEvent>, config: Config) -> Result<()> {
    loop {
        let stream_event = stream_events.recv().context("getting stream event")?;

        match stream_event {
            StreamEvent::ChangeHelixTheme { username, theme } => {
                println!("{username} changed the Helix theme to {theme}");

                change_helix_theme::change_helix_theme(&config.helix_config_path, &theme)?
            }
            StreamEvent::Unknown => eprintln!("got an unknown stream event"),
        }
    }
}
