use axum::extract::ws::{Message, WebSocket};
use futures::stream::{SplitSink, SplitStream};

pub type Token = String;
pub type RoomId = String;
pub type UserId = String;
pub type TweetId = u32;

pub type TxStream = SplitSink<WebSocket, Message>;
pub type RxStream = SplitStream<WebSocket>;

#[derive(Debug)]
pub struct Tweet {
    username: String,
    message: String,
}

impl Tweet {
    pub fn new(username: &str, message: &str) -> Self {
        Self {
            username: Self::sanitize_username(username),
            message: Self::sanitize_tweet(message),
        }
    }

    fn sanitize_username(input: &str) -> String {
        input
            .chars()
            .filter(|x| x.is_ascii_alphanumeric())
            .collect()
    }

    fn sanitize_tweet(input: &str) -> String {
        // Or, alternatively, it could use an actual html sanitizer;
        const ALLOWED: &str = "+-*/,.&:;()";
        input
            .chars()
            .filter(|x| x.is_alphanumeric() || ALLOWED.contains(*x))
            .collect()
    }
}
