extern crate futures;
extern crate telegram_bot_fork;
extern crate tokio;
extern crate tokio_timer;

use futures::{future::lazy, Future, Stream};
use std::{
    env,
    ops::Add,
    time::{Duration, Instant},
};
use telegram_bot_fork::*;
use tokio::runtime::current_thread::{self, Runtime};
use tokio_timer::Delay;

fn test(api: Api, message: Message) {
    let timeout = |n| Delay::new(Instant::now().add(Duration::from_secs(n))).from_err();
    let api_future = || Ok(api.clone());

    let future = api
        .send(message.location_reply(0.0, 0.0).live_period(60))
        .join(api_future())
        .join(timeout(2))
        .and_then(|((message, api), _)| api.send(message.edit_live_location(10.0, 10.0)))
        .join(api_future())
        .join(timeout(4))
        .and_then(|((message, api), _)| api.send(message.edit_live_location(20.0, 20.0)))
        .join(api_future())
        .join(timeout(6))
        .and_then(|((message, api), _)| api.send(message.edit_live_location(30.0, 30.0)));

    current_thread::spawn(future.then(|_| Ok(())));
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
                            if let MessageKind::Text { ref data, .. } = message.kind {
                                match data.as_str() {
                                    "/livelocation" => test(api.clone(), message.clone()),
                                    _ => (),
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }

                Ok(())
            })
        }))
        .unwrap();
}
