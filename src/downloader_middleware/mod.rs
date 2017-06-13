extern crate hyper;
use downloader::{Request, RequestContent, Response, Method, Error};
// use self::hyper::client::response::Response;


pub enum MiddleWareResult{
    FinalRequest(Request),
    IntermediateRequest(Request),
    Response(Response),
    Ignore,
}

pub enum MiddleWareExceptionResult{
    Continue,
    Request(Request),
    Response(Response),
}

pub trait DownloaderMiddleware: Send{
    /// Process request before it's sent to the downloader. Typical
    /// application includes setting the timeout and redirection policy.
    fn process_request(&mut self, request: Request) -> MiddleWareResult;
    fn process_response(&mut self, request_content: &RequestContent, response: Response) -> MiddleWareResult;
    fn process_exception(&mut self, request_content: &RequestContent, error: &Error) -> MiddleWareExceptionResult;
}
