use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::internal::types::{Token, TweetId};

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct PubTweet {
    token: Token,
    content: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct PubRetweet {
    references: TweetId,
    #[serde(flatten)]
    tweet: PubTweet,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct PubLike {
    token: Token,
    tweet: TweetId,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct PubReport {
    token: Token,
    tweet: TweetId,
}

/// Events published by the user.
#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub enum PubEvent {
    Tweet(PubTweet),
    Retweet(PubRetweet),
    Like(PubLike),
    Report(PubReport),
}

impl PubEvent {
    pub fn token(&self) -> Token {
        match self {
            PubEvent::Tweet(x) => x.token.clone(),
            PubEvent::Retweet(x) => x.tweet.token.clone(),
            PubEvent::Like(x) => x.token.clone(),
            PubEvent::Report(x) => x.token.clone(),
        }
    }
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct SubTweet {
    id: TweetId,
    username: String,
    content: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct SubRetweet {
    references: TweetId,
    #[serde(flatten)]
    tweet: SubTweet,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct SubBlocked {
    username: String,
    /// Left, banned or kicked.
    reason: String,
}

/// Events user subscribes to.
#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub enum SubEvent {
    Tweet(SubTweet),
    Retweet(SubRetweet),
    Blocked(SubBlocked),
}
