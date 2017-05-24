extern crate hyper;
extern crate url;
use self::hyper::Client;
use self::hyper::client::{Body, RequestBuilder};
use self::hyper::header::{Header, HeaderFormat};
use self::url::Url;


pub enum Method {
    Get,
    Post,
}

pub struct Request<'a> {
    pub url: Url,
    pub method: Method,
    pub body: Option<Body<'a>>,
    pub client: Client,
}

impl<'a> Request<'a> {
    fn download(&self) {
        let url_str = self.url.as_str();
        let client: RequestBuilder;
        match self.method {
            ref Get => {
                client = self.client.get(url_str);
            },
            ref Post => {
                client = self.client.post(url_str);
            },
        }
        // if let Some(body) = self.body{
            unsafe {
                client.body = self.body;
            }
        // }
    }
}
