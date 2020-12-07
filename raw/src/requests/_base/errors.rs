use failure;
use std::result;
use types::*;

pub type Result<T, E = failure::Error> = result::Result<T, E>;

#[derive(Debug, Fail)]
#[fail(description = "telegram-bot-raw error")]
pub enum Error {
    #[fail(display = "empty body")]
    EmptyBody,
    #[fail(display = "telegram error: {}", description)]
    TelegramError {
        description: String,
        parameters: Option<ResponseParameters>,
    },
    #[fail(display = "detached error: {}", _0)]
    DetachedError(String),
}
