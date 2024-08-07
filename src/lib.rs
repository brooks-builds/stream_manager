pub mod config;
mod hello_queue;
mod state;

use change_alacritty_font::{change_alacritty_font, validate_font};
use change_helix_theme::{select_random_theme, validate_theme};
use config::Config;
use eyre::{Context, Result};
use frontend::events::Events;
use state::State;
use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    time::Duration,
};
use tokio::{spawn, sync::mpsc::Sender, time::sleep};
use twitch_events_listener::stream_event::StreamEvent;

use crate::hello_queue::hello_queue;

pub async fn run(
    stream_events: Receiver<StreamEvent>,
    config: Config,
    frontend_events: Sender<Events>,
) -> Result<()> {
    let state = Arc::new(Mutex::new(State::default()));

    loop {
        let state = state.clone();
        let frontend_events = frontend_events.clone();
        let config = config.clone();
        let stream_event = stream_events
            .recv()
            .context("getting stream event")
            .unwrap();

        tokio::spawn(async move {
            let state = state.clone();

            match stream_event {
                StreamEvent::ChangeHelixTheme { username, theme } => {
                    let theme = if theme.to_lowercase() == "random" {
                        select_random_theme()
                    } else {
                        &theme
                    };

                    if validate_theme(&theme) {
                        frontend_events
                            .send(Events::ThemeChanged {
                                username,
                                theme: theme.to_owned(),
                            })
                            .await
                            .context("sending theme event to frontend")
                            .unwrap();

                        change_helix_theme::change_helix_theme(&config.helix_config_path, &theme)
                            .unwrap();
                    }
                }
                StreamEvent::ChangeFont { username, font } => {
                    if validate_font(&font) {
                        let path = config.alacritty_font_config_path.clone();
                        let previous_font = change_alacritty_font(&path, &font, &username, false)
                            .context("changing font")
                            .unwrap();
                        let reset_font_in = Duration::from_secs(60 * 15);

                        frontend_events
                            .send(Events::FontChanged {
                                username: username.clone(),
                                font: font.clone(),
                                time_left: reset_font_in.clone(),
                            })
                            .await
                            .context("sending font changed message")
                            .unwrap();

                        spawn(async move {
                            let mut reset_font_in = reset_font_in;
                            let one_second = Duration::from_secs(1);

                            while !reset_font_in.is_zero() {
                                frontend_events
                                    .send(Events::FontChanged {
                                        username: username.clone(),
                                        font: font.clone(),
                                        time_left: reset_font_in.clone(),
                                    })
                                    .await
                                    .context("sending time left for font")
                                    .unwrap();

                                reset_font_in -= one_second;
                                sleep(one_second).await;
                            }

                            change_alacritty_font(&path, &previous_font, "brookzerker", true)
                                .unwrap();

                            frontend_events
                                .send(Events::FontChanged {
                                    username: "Brookzerker".to_owned(),
                                    font: previous_font,
                                    time_left: Duration::ZERO,
                                })
                                .await
                                .context("sending font reset")
                                .unwrap();
                        });
                    }
                }
                StreamEvent::Unknown => eprintln!("got an unknown stream event"),
                StreamEvent::AdBreakBegin { duration } => {
                    frontend_events
                        .send(Events::TwitchAds(duration))
                        .await
                        .context("sending duration of twitch ads")
                        .unwrap();
                }
                StreamEvent::ChatMessage { username } => {
                    hello_queue(username, state, frontend_events)
                        .await
                        .context("running hello queue")
                        .unwrap();
                }
            }
        })
        .await
        .context("running everything")
        .unwrap();
    }
}
