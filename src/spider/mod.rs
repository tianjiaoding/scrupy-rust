extern crate hyper;
extern crate url;
use self::hyper::client::response::Response;
use self::hyper::Client;
use http::{Request, Method};
use self::url::Url;

pub trait Spider{
    type ItemOut;
    fn name(&self) -> &str;
    fn allowed_domains(&self) -> &[String];
    fn start_urls(&self) -> &[String];
    fn start_requests(&self) -> Vec<Request>{
        let start_urls = self.start_urls();
        let mut requests: Vec<Request> = vec![];
        for start_url in start_urls {
            match Url::parse(&start_url){
                Ok(url) => {
                    requests.push(Request{
                        url: url,
                        method: Method::Get,
                        body: None,
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
    fn log(&self, _str: &str){
        println!("{}", _str);
    }
    fn parse(&mut self, response: Response) -> (Option<Vec<i32>>, Option<Vec<i32>>);
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
