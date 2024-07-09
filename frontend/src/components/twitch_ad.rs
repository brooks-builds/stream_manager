use std::time::Duration;

use anathema::{
    component::Component,
    state::{State, Value},
};

pub struct TwitchAds;

impl Component for TwitchAds {
    type State = TwitchAdsState;

    type Message = Duration;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
    ) {
        let time_remaining = message.as_secs();

        state.time_remaining.set(time_remaining);
    }
}

#[derive(State)]
pub struct TwitchAdsState {
    pub time_remaining: Value<u64>,
}
