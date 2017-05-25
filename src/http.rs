extern crate hyper;
extern crate url;
use self::hyper::Client;
use self::hyper::client::{Body, RequestBuilder};
use self::hyper::header::{Header, HeaderFormat};
use self::url::Url;
use std::marker::PhantomData;


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
    pub client: Client,
    phantom: PhantomData<&'a B>,
}

impl<'a, B: 'a + Into<Body<'a>>> Request<'a, B> {
    fn download(self) {
        // let url_str = self.url.as_str();
        // let client: RequestBuilder;
        // match self.method {
        //     ref Get => {
        //         client = self.client.get(url_str);
        //     },
        //     ref Post => {
        //         client = self.client.post(url_str);
        //     },
        // }
        // let mut client = self.client.get("haha.com");
    //     if let Some(body) = self.body{
        // let client = client.body(self.body.unwrap());
    //     }
        let cl1 = self.client;
        let mut cl3 = cl1.get("haha.com");
        let body2 = self.body.unwrap();
        // let body3 = body2.into();
        // {
            cl3 = cl3.body(body2);
        // }
    }
}
