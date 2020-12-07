extern crate futures;
extern crate telegram_bot_fork;
extern crate tokio;

use futures::{future::lazy, Stream};
use std::env;
use telegram_bot_fork::*;
use tokio::runtime::current_thread::Runtime;

fn process(api: Api, message: Message) {
    if let MessageKind::Text { ref data, .. } = message.kind {
        match data.as_str() {
            "/pin" => message
                .reply_to_message
                .map(|message| api.spawn(message.pin()))
                .unwrap_or(()),
            "/unpin" => api.spawn(message.chat.unpin_message()),
            _ => (),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    runtime
        .block_on(lazy(|| {
            let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let api = Api::new(token).unwrap();

            let stream = api.stream().then(|mb_update| {
                let res: Result<_, ()> = Ok(mb_update);
                res
            });

            stream.for_each(move |update| {
                match update {
                    Ok(update) => {
                        if let UpdateKind::Message(message) = update.kind {
                            process(api.clone(), message)
                        }
                    }
                    Err(_) => {}
                }

                Ok(())
            })
        }))
        .unwrap();
}
