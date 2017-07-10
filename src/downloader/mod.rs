extern crate hyper;
extern crate url;
use self::hyper::Client;
use self::hyper::client::RequestBuilder;
use self::hyper::header::Headers;
use self::hyper::status::StatusCode;
use self::hyper::client::response::Response as HpResp;
use self::hyper::error::Error as HpErr;
use self::url::Url;
use std::sync::mpsc::channel;
use std::thread;
use std::io::{Read, Error as ReadErr};
use std::io::ErrorKind;
use std::time::Duration;
use std::sync::Arc;
use engine::Crawler;

/// Download error occured when issueing a request.
pub enum DownloadError{
    /// The status code is not Ok.
    BadStatus(HpResp),
    /// Special read error timeout.
    TimedOut(HpResp),
    /// Other read error except for timeout.
    ReadError(HpResp, ReadErr),
    /// Errors that can occur parsing HTTP streams.
    BadRequest(HpErr),
}

#[derive(Clone)]
/// 
pub enum Method {
    Get,
    Post,
}

pub struct Response {
    pub url: Url,
    pub headers: Headers,
    pub body: Vec<u8>,//TODO: use a better linear container.
}

#[derive(Clone)]
pub struct RequestContent{
    pub url: Url,
    pub method: Method,
    pub body: Option<String>,
}
pub struct Request
{
    pub content: RequestContent,
    pub client: Client,
}

impl Request {
    pub fn download(self)-> Result<Response, DownloadError> {
        let url = self.content.url.clone();
        let mut client: RequestBuilder;
        match self.content.method {
            Method::Get => {
                client = self.client.get(url);
            },
            Method::Post => {
                client = self.client.post(url);
            },
        }
        if let Some(ref body) = self.content.body{
            client = client.body(body);
        }
        let response = client.send();
        match response{
            Ok(mut response) => {
                if let StatusCode::Ok = response.status{
                    let mut buffer = vec![];
                    match response.read_to_end(&mut buffer){
                        Ok(_) => {
                            Ok(Response{
                                url: response.url.clone(),
                                headers: response.headers.clone(),
                                body: buffer,
                            })
                        }
                        Err(e) => {
                            match e.kind(){
                                ErrorKind::TimedOut => {
                                    Err(DownloadError::TimedOut(response))
                                }
                                _ => {
                                    Err(DownloadError::ReadError(response, e))
                                }
                            }
                        }
                    }
                }
                else{
                    Err(DownloadError::BadStatus(response))
                }
            },
            Err(e) => Err(DownloadError::BadRequest(e))
        }
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
