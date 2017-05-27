extern crate hyper;
extern crate url;
use self::hyper::Client;
use self::hyper::client::{Body, RequestBuilder};
use self::hyper::client::response::Response;
use self::hyper::header::{Header, HeaderFormat};
use self::hyper::error::Error;
use self::url::Url;


pub enum Method {
    Get,
    Post,
}

pub struct Request<'a, B>
where B: 'a + Into<Body<'a>>
{
    pub url: Url,
    pub method: Method,
    pub body: Option<B>,
    pub client: &'a Client,
}

impl<'a, B: 'a + Into<Body<'a>>> Request<'a, B> {
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
        if let Some(body) = self.body{
            client = client.body(body);
        }
        client.send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_download() {
        let url = Url::parse("http://www.baidu.com").unwrap();
        let client = Client::new();
        let request:Request<&str> = Request{
            url: url,
            method: Method::Get,
            body: None,
            client: &client,
        };
        request.download().unwrap();
    }
}
