use std::time::Duration;

use anathema::{
    component::Component,
    state::{State, Value},
};

pub struct ThemeComponent;

impl Component for ThemeComponent {
    type State = ThemeState;

    type Message = ThemeMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
    ) {
        state.name.set(message.theme);
        state.username.set(message.username);
        state.keep_for.set(message.keep_for.as_secs());
    }
}

#[derive(State)]
pub struct ThemeState {
    pub name: Value<String>,
    pub username: Value<String>,
    pub keep_for: Value<u64>,
}

pub struct ThemeMessage {
    pub username: String,
    pub theme: String,
    pub keep_for: Duration,
}
