mod components;
pub mod events;

const KEEP_HX_THEME_FOR: Duration = Duration::from_secs(60 * 5);
const ONE_SECOND: Duration = Duration::from_secs(1);

use std::{fs::read_to_string, thread::sleep, time::Duration};

use anathema::{
    backend::tui::TuiBackend,
    runtime::Runtime,
    state::{List, Value},
    templates::Document,
};
use components::{
    hello_queue::{HelloQueue, HelloQueueState},
    theme::{ThemeComponent, ThemeMessage, ThemeState},
    twitch_ad::{TwitchAds, TwitchAdsState},
};
use events::Events;
use eyre::{Context, Result};
use tokio::{sync::mpsc::Receiver, task::spawn_blocking};

pub fn run(mut events: Receiver<Events>) -> Result<()> {
    let hello_queue_template = read_to_string("templates/hello_queue_template.aml")
        .context("loading the hello queue template")?;
    let index_template = read_to_string("templates/index.aml").context("loading index template")?;
    let twitch_ads_template =
        read_to_string("templates/ads.aml").context("loading ads template")?;
    let theme_template = read_to_string("templates/theme.aml").context("loading theme template")?;

    let doc = Document::new(index_template);
    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .enable_mouse()
        .hide_cursor()
        .finish()
        .context("building the backend")?;

    let mut runtime_builder = Runtime::builder(doc, backend);

    let hello_queue_component = runtime_builder.register_component(
        "helloqueue",
        hello_queue_template,
        HelloQueue,
        HelloQueueState {
            names: List::empty(),
        },
    );

    let twitch_ads_component = runtime_builder.register_component(
        "twitchads",
        twitch_ads_template,
        TwitchAds,
        TwitchAdsState {
            time_remaining: Value::new(0),
        },
    );

    let theme_component = runtime_builder.register_component(
        "theme",
        theme_template,
        ThemeComponent,
        ThemeState {
            name: Value::new("Adwaita-dark".to_owned()),
            username: Value::new("".to_owned()),
            keep_for: Value::new(0),
        },
    );

    let mut runtime = runtime_builder.finish().context("Creating runtime")?;
    let emitter = runtime.emitter();

    spawn_blocking(move || loop {
        let Some(event) = events.blocking_recv() else {
            continue;
        };

        match event {
            Events::HelloMessage(username) => {
                emitter
                    .emit(hello_queue_component, username)
                    .context("emitting username to hello queue component")
                    .unwrap();
            }
            Events::TwitchAds(duration) => {
                emitter
                    .emit(twitch_ads_component, duration.clone())
                    .context("twitch ads just started")
                    .unwrap();

                {
                    let mut duration = duration;
                    let emitter = emitter.clone();

                    spawn_blocking(move || {
                        while duration.as_secs() > 0 {
                            duration -= ONE_SECOND;
                            emitter
                                .emit(twitch_ads_component, duration)
                                .context("sending count down ad time")
                                .unwrap();

                            sleep(ONE_SECOND);
                        }
                    });
                }
            }
            Events::ThemeChanged { username, theme } => {
                emitter
                    .emit(
                        theme_component,
                        ThemeMessage {
                            username: username.clone(),
                            theme: theme.clone(),
                            keep_for: KEEP_HX_THEME_FOR,
                        },
                    )
                    .context("Sending theme change info to component")
                    .unwrap();

                {
                    let emitter = emitter.clone();

                    let handle = spawn_blocking(move || {
                        let mut duration = KEEP_HX_THEME_FOR;

                        while !duration.is_zero() {
                            emitter
                                .emit(
                                    theme_component,
                                    ThemeMessage {
                                        username: username.clone(),
                                        theme: theme.clone(),
                                        keep_for: duration,
                                    },
                                )
                                .context("updating theme duration")
                                .unwrap();

                            duration -= ONE_SECOND;

                            sleep(ONE_SECOND);
                        }
                    });
                }
            }
        };
    });

    runtime
        .run()
        .context("Running the tui")
        .context("running anathema")
        .unwrap();

    Ok(())
}
