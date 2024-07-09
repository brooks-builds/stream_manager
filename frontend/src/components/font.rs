use std::time::Duration;

use anathema::{
    component::Component,
    state::{State, Value},
};

pub struct Font;

impl Component for Font {
    type State = FontState;

    type Message = FontMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
    ) {
        state.name.set(message.font);
        state.username.set(message.username);
        state.time_left.set(message.time_left.as_secs());
    }
}

#[derive(State)]
pub struct FontState {
    pub name: Value<String>,
    pub username: Value<String>,
    pub time_left: Value<u64>,
}

pub struct FontMessage {
    pub username: String,
    pub font: String,
    pub time_left: Duration,
}
