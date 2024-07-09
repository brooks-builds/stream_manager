use std::time::Duration;

#[derive(Debug)]
pub enum Events {
    HelloMessage(String),
    TwitchAds(Duration),
    ThemeChanged {
        username: String,
        theme: String,
    },
    FontChanged {
        username: String,
        font: String,
        time_left: Duration,
    },
}
