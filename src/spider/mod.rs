extern crate hyper;
use self::hyper::Client;
use http::Request;

pub trait Spider{
    fn name(&self) -> &str;
    fn allowed_domains(&self) -> &[String];
    fn start_urls(&self) -> &[String];
    fn start_requests(&self) -> Vec<Request>;
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
