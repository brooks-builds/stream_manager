use anathema::{
    component::Component,
    state::{List, State, Value},
};

pub struct HelloQueue;

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
            .first(|_el, _att| {
                found = true;
            });

        if found {
            state.names.pop_front();
        }
    }
}

#[derive(State)]
pub struct HelloQueueState {
    pub names: Value<List<String>>,
}
