//! Spider module

extern crate hyper;
extern crate url;
use self::hyper::Client;
use downloader::{Request, Response, Method, RequestContent};
use self::url::Url;

pub trait Spider: Send + Sync{
    type ItemType;
    /// Returns the name of the spider.
    fn name(&self) -> &str;
    /// Returns the allowed domains of the spider.
    fn allowed_domains(&self) -> &[String];
    /// Returns a set of urls for the spider to start with.
    fn start_urls(&self) -> &[String];
    /// Returns a set of start requests for the spider to start with. By default
    /// It will call start_urls() to get start urls and issue http get requests
    /// to those urls.
    fn start_requests(&self) -> Vec<Request>{
        let start_urls = self.start_urls();
        let mut requests: Vec<Request> = vec![];
        for start_url in start_urls {
            match Url::parse(&start_url){
                Ok(url) => {
                    requests.push(Request{
                        content: RequestContent{
                            url: url,
                            method: Method::Get,
                            body: None,
                        },
                        client: Client::new(),
                    });
                },
                Err(e) => {
                    self.log(&format!("{}", e));
                }
            }
        }
        requests
    }
    /// Logging functions of the spider. By default it will print error to `stdout`.
    fn log(&self, _str: &str){
        println!("{}", _str);
    }
    /// Parse the `Response` and get a set of new `Request`s and items.
    fn parse(&self, response: Response) -> (Vec<Request>, Vec<Self::ItemType>);
}

#[cfg(test)]
mod tests {
    use super::Spider;
    struct LittleSpider{
        domains: Vec<String>,
    }
    impl Spider for LittleSpider {
        fn name(&self) -> &str {
            "Little Spider"
        }
        fn allowed_domains(&self) -> &[String]{
            self.domains.as_slice()
        }
        fn start_urls(&self) -> &[String]{
            self.domains.as_slice()
        }
        fn log(&self, _str: &str){
            println!("{}", _str);
        }
    }

    #[test]
    fn spider_name() {
        let spider = LittleSpider{ domains: vec!["google.com".to_owned(), "shanghaitech.edu.cn".to_owned()] };
        let b = spider.name();
        assert_eq!(b, "Little Spider");
    }

    #[test]
    fn spider_domains() {
        let spider = LittleSpider{ domains: vec!["google.com".to_owned(), "shanghaitech.edu.cn".to_owned()] };
        let b = spider.allowed_domains();
        assert_eq!(b, vec!["google.com".to_owned(), "shanghaitech.edu.cn".to_owned()].as_slice());
    }
}
