extern crate failure;
extern crate futures;
extern crate telegram_bot_fork;
extern crate tokio;

use failure::Error;
use futures::{future::lazy, Future};
use std::env;
use telegram_bot_fork::*;
use tokio::runtime::current_thread::{self, Runtime};

fn main() {
    let mut runtime = Runtime::new().unwrap();
    runtime
        .block_on(lazy(|| {
            let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let api = Api::new(token).unwrap();

            current_thread::spawn(api.send(GetMe).then(|r| {
                println!("{:?}", r);

                Ok(())
            }));

            Ok::<_, Error>(())
        }))
        .unwrap();
}
