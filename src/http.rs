extern crate hyper;
use self::hyper::Client;
use self::hyper::client::Body;
use self::hyper::header::{Header, HeaderFormat};


enum Method {
    Get,
    Post,
}

pub struct Request<'a, B: Into<Body<'a>>> {
    url: String,
    method: Method,
    body: B,
}
