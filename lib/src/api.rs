use std::{rc::Rc, time::Duration};

use futures::{future::result, Future};
use tokio;
use tokio_timer;

use telegram_bot_fork_raw::{Request, ResponseType};

use connector::Connector;
use future::{NewTelegramFuture, TelegramFuture};
use stream::{NewUpdatesStream, UpdatesStream};
#[cfg(feature = "hyper_connector")]
use {connector::default_connector, errors::Result};

/// Main type for sending requests to the Telegram bot API.
#[derive(Clone)]
pub struct Api {
    url: Option<String>,
    inner: Rc<ApiInner>,
}

struct ApiInner {
    token: String,
    connector: Box<Connector>,
}

impl Api {
    /// Start construction of the `Api` instance.
    ///
    /// # Examples
    ///
    /// Using default connector.
    ///
    /// ```rust
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// use telegram_bot_fork::Api;
    ///
    /// # fn main() {
    /// # let telegram_token = "token";
    /// let api = Api::new(telegram_token).unwrap();
    /// # }
    /// ```
    ///
    /// Using custom connector.
    ///
    ///
    /// ```rust
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// # #[cfg(feature = "hyper_connector")]
    /// # fn main() {
    /// use telegram_bot_fork::{connector::hyper, Api};
    ///
    /// # let telegram_token = "token";
    /// let api = Api::with_connector(telegram_token, hyper::default_connector().unwrap());
    /// # }
    ///
    /// # #[cfg(not(feature = "hyper_connector"))]
    /// # fn main() {}
    /// ```
    #[cfg(feature = "hyper_connector")]
    pub fn new<T: AsRef<str>>(token: T) -> Result<Self> {
        Ok(Self::with_connector(token, default_connector()?))
    }

    pub fn with_connector<T: AsRef<str>>(token: T, connector: Box<Connector>) -> Self {
        Api {
            url: None,
            inner: Rc::new(ApiInner {
                token: token.as_ref().to_string(),
                connector,
            }),
        }
    }

    pub fn set_url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.url = Some(url.as_ref().into());

        self
    }

    /// Create a stream which produces updates from the Telegram server.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// # use telegram_bot_fork::Api;
    /// # fn main() {
    /// # let api: Api = Api::new("token").unwrap();
    /// use futures::Stream;
    ///
    /// let future = api.stream().for_each(|update| {
    ///     println!("{:?}", update);
    ///     Ok(())
    /// });
    /// # }
    /// ```
    pub fn stream(&self) -> UpdatesStream {
        UpdatesStream::new(self.clone())
    }

    /// Send a request to the Telegram server and do not wait for a response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// # use futures::Future;
    /// # use telegram_bot_fork::{Api, GetMe, ChatId};
    /// # use telegram_bot_fork::prelude::*;
    /// #
    /// # fn main() {
    /// # let telegram_token = "token";
    /// # let api = Api::new(telegram_token).unwrap();
    /// # if false {
    /// let chat = ChatId::new(61031);
    /// api.spawn(chat.text("Message"))
    /// # }
    /// # }
    pub fn spawn<Req: Request>(&self, request: Req) {
        tokio::executor::current_thread::spawn(self.send(request).then(|_| Ok(())));
    }

    /// Send a request to the Telegram server and wait for a response, timing out after `duration`.
    /// Future will resolve to `None` if timeout fired.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// # use futures::Future;
    /// # use telegram_bot_fork::{Api, GetMe};
    /// #
    /// # fn main() {
    /// # let telegram_token = "token";
    /// # let api = Api::new(telegram_token).unwrap();
    /// # if false {
    /// use std::time::Duration;
    ///
    /// let future = api.send_timeout(GetMe, Duration::from_secs(5));
    /// future.and_then(|me| Ok(assert!(me.is_some())));
    /// # }
    /// # }
    /// ```
    pub fn send_timeout<Req: Request>(
        &self,
        request: Req,
        duration: Duration,
    ) -> TelegramFuture<Option<<Req::Response as ResponseType>::Type>> {
        let timeout_future = tokio_timer::sleep(duration).from_err().map(|()| None);
        let send_future = self.send(request).map(Some);

        let future = timeout_future
            .select(send_future)
            .map(|(item, _next)| item)
            .map_err(|(item, _next)| item);

        TelegramFuture::new(Box::new(future))
    }

    /// Send a request to the Telegram server and wait for a response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// # use futures::Future;
    /// # use telegram_bot_fork::{Api, GetMe};
    /// #
    /// # fn main() {
    /// # let telegram_token = "token";
    /// # let api = Api::new(telegram_token).unwrap();
    /// # if false {
    /// let future = api.send(GetMe);
    /// future.and_then(|me| Ok(println!("{:?}", me)));
    /// # }
    /// # }
    /// ```
    pub fn send<Req: Request>(
        &self,
        request: Req,
    ) -> TelegramFuture<<Req::Response as ResponseType>::Type> {
        let request = request.serialize().map_err(From::from);

        let request = result(request);

        let api = self.clone();
        let response = request.and_then(move |request| {
            let url = &api.url;
            let token = &api.inner.token;
            api.inner
                .connector
                .request(url.as_ref().map(String::as_str), token, request)
        });

        let future = response
            .and_then(move |response| Req::Response::deserialize(response).map_err(From::from));

        TelegramFuture::new(Box::new(future))
    }
}
