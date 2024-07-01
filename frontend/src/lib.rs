mod components;
pub mod events;

use std::fs::read_to_string;

use anathema::{
    backend::tui::TuiBackend,
    runtime::Runtime,
    state::{List, Value},
    templates::Document,
};
use components::{
    hello_queue::{HelloQueue, HelloQueueState},
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
    let doc = Document::new(index_template);
    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .enable_mouse()
        .hide_cursor()
        .finish()
        .context("building the backend")?;

    let mut runtime_builder = Runtime::new(doc, backend);

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

    let mut runtime = runtime_builder.finish().context("Creating runtime")?;
    let emitter = runtime.emitter();

    emitter
        .emit("testing 123", hello_queue_component)
        .context("testing emitter")
        .unwrap();

    spawn_blocking(move || loop {
        let Some(event) = events.blocking_recv() else {
            continue;
        };

        match event {
            Events::HelloMessage(username) => {
                emitter
                    .emit(username, hello_queue_component)
                    .context("emitting username to hello queue component")
                    .unwrap();
            }
            Events::TwitchAds(duration) => emitter
                .emit(duration, twitch_ads_component)
                .context("twitch ads just started")
                .unwrap(),
        };
    });

    runtime
        .run()
        .context("Running the tui")
        .context("running anathema")
        .unwrap();

    Ok(())
}
