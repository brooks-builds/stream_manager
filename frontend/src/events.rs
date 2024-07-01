use std::time::Duration;

#[derive(Debug)]
pub enum Events {
    HelloMessage(String),
    TwitchAds(Duration),
}
