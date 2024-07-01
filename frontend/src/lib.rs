pub mod events;

use std::fs::read_to_string;

use anathema::{
    backend::tui::TuiBackend,
    component::Component,
    runtime::Runtime,
    state::{List, State, Value},
    templates::Document,
};
use events::Events;
use eyre::{Context, Result};
use tokio::{sync::mpsc::Receiver, task::spawn_blocking};

pub fn run(mut events: Receiver<Events>) -> Result<()> {
    let hello_queue_template = read_to_string("templates/hello_queue_template.aml")
        .context("loading the hello queue template")?;
    let index_template = read_to_string("templates/index.aml").context("loading index template")?;
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
        };
    });

    runtime
        .run()
        .context("Running the tui")
        .context("running anathema")
        .unwrap();

    Ok(())
}

struct HelloQueue;

impl Component for HelloQueue {
    type State = HelloQueueState;

    type Message = String;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
    ) {
        state.names.push(message);
    }

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
    ) {
        if !mouse.lsb_up() {
            return;
        };

        let mut found = false;

        elements
            .query(&state)
            .at_position(mouse.pos())
            .first(|el, att| {
                found = true;
            });

        if found {
            state.names.pop_front();
        }
    }
}

#[derive(State)]
struct HelloQueueState {
    pub names: Value<List<String>>,
}
