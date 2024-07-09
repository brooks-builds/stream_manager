use eyre::Result;
use frontend::events::Events;
use tokio::sync::mpsc::channel;

fn main() -> Result<()> {
    let (_, events_receiver) = channel::<Events>(1);

    frontend::run(
        events_receiver,
        "adwaita-dark".to_owned(),
        "Iosevka".to_owned(),
    )
}
