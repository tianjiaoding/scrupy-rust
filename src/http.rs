extern crate hyper;
extern crate url;
use self::hyper::Client;
use self::hyper::client::RequestBuilder;
use self::hyper::client::response::Response as Hpresp;
use self::hyper::header::{Headers,HeaderFormat};
use self::hyper::status::StatusCode;
use self::hyper::error::Error as Hperr;
use self::url::Url;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;


pub enum Error{
    Timeout,
    HyperError(Hperr),
}

pub enum Method {
    Get,
    Post,
}

pub struct Response {
    pub headers: Headers,
    pub url: Url,
    pub body: Vec<u8>,//TODO: use a better linear container.
}

pub struct Request
{
    pub url: Url,
    pub method: Method,
    pub body: Option<String>,
    pub client: Client,
}

impl Request {
    fn download(self)->Result<Response, Error> {
        let url_str = self.url.as_str();
        let mut client: RequestBuilder;
        match self.method {
            Method::Get => {
                client = self.client.get(url_str);
            },
            Method::Post => {
                client = self.client.post(url_str);
            },
        }
        if let Some(ref body) = self.body{
            client = client.body(body);
        }
        let (tx, rx) = channel();
        let tx_ = tx.clone();

        thread::spawn(move || {
            let response = client.send();
            let _ = tx.send(
                match response{
                    Ok(mut response) => {
                        if let StatusCode::Ok
                    },
                    Err(err) => Err(Error::HyperError(err)),
                }
            );
        });
        rx.recv().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_download() {
        let url = Url::parse("http://www.baidu.com").unwrap();
        let client = Client::new();
        let request:Request = Request{
            url: url,
            method: Method::Get,
            body: None,
            client: client,
        };
        request.download().unwrap();
    }
}
