use std::sync::{Arc, Mutex};

use eyre::{eyre, Context, Result};
use frontend::events::Events;
use tokio::sync::mpsc::Sender;

use crate::state::State;

pub async fn hello_queue(
    username: String,
    wrapped_state: Arc<Mutex<State>>,
    frontend_events: Sender<Events>,
) -> Result<()> {
    {
        let mut state = wrapped_state
            .lock()
            .map_err(|error| eyre!("got an error while locking state: {error}"))?;

        if state.seen_usernames.contains(&username) {
            return Ok(());
        }

        state.seen_usernames.push(username.clone());
    }

    frontend_events
        .send(Events::HelloMessage(username))
        .await
        .context("sending username to frontend")?;

    Ok(())
}
