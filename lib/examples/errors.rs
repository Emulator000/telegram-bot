extern crate futures;
extern crate telegram_bot_fork;
extern crate tokio;

use futures::{future::lazy, Stream};
use std::env;
use telegram_bot_fork::*;
use tokio::runtime::current_thread::Runtime;

fn main() {
    let mut runtime = Runtime::new().unwrap();
    runtime
        .block_on(lazy(|| {
            let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let api = Api::new(token).unwrap();

            // Convert stream to the stream with errors in result
            let stream = api.stream().then(|mb_update| {
                let res: Result<_, ()> = Ok(mb_update);
                res
            });

            // Print update or error for each update.
            stream.for_each(|mb_update| {
                println!("{:?}", mb_update);

                Ok(())
            })
        }))
        .unwrap();
}
