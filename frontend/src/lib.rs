use std::{fs::read_to_string, thread::spawn};

use anathema::{
    backend::tui::TuiBackend,
    component::Component,
    runtime::{Emitter, Runtime},
    state::{State, Value},
    templates::Document,
    widgets::components::ComponentId,
};
use eyre::{Context, Result};

pub fn run() -> Result<()> {
    let hello_queue_template = read_to_string("templates/hello_queue_template.aml")
        .context("loading the hello queue template")?;
    let index_template = read_to_string("templates/index.aml").context("loading index template")?;
    let mut doc = Document::new(index_template);
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .context("building the backend")?;

    let mut runtime_builder = Runtime::new(doc, backend);

    let hello_queue_component = runtime_builder.register_component(
        "helloqueue",
        hello_queue_template,
        HelloQueue,
        HelloQueueState {
            name: "hi there".to_owned().into(),
        },
    );

    let mut runtime = runtime_builder.finish().context("Creating runtime")?;
    let mut emitter = runtime.emitter();

    let thread = spawn(move || {
        send_messages(emitter, hello_queue_component);
    });

    runtime.run().context("Running the tui")?;

    thread.join().unwrap();

    Ok(())
}

// Send a string to a recipient every second
fn send_messages(emitter: Emitter, recipient: ComponentId) {
    let mut counter = 0;

    loop {
        emitter.emit(format!("{counter} message"), recipient);
        counter += 1;
        std::thread::sleep_ms(2000);
    }
}

struct HelloQueue;

impl Component for HelloQueue {
    type State = HelloQueueState;

    type Message = String;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
    ) {
        state.name = message.into();
    }
}

#[derive(State)]
struct HelloQueueState {
    pub name: Value<String>,
}
