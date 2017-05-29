extern crate hyper;
use http::{Request, Method};
use self::hyper::client::response::Response;

pub trait DownloaderMiddleware{
    type Spider;
    fn process_request(request: Request, spider: &Self::Spider);
    fn process_response(request: Request, response: Response, spider: &Self::Spider);
    fn process_exception(request: Request, exception: Exception, spider: &Self::Spider);
}
