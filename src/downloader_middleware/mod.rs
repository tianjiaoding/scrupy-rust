extern crate hyper;
use downloader::{Request, RequestContent, Response, Method, DownloadError};
// use self::hyper::client::response::Response;

/// Indicates what to do after the `Request` or `Response` has been processed.
pub enum MiddleWareResult{
    /// Scrupy will stop calling process_request methods and reschedule the returned request.
    FinalRequest(Request),
    /// Scrupy will continue processing this request, executing all other middlewares until, finally, the appropriate downloader handler is called the request performed (and its response downloaded).
    IntermediateRequest(Request),
    /// Scrupy won’t bother calling any other process_request() or process_exception() methods, or the appropriate download function; it’ll return that response. The process_response() methods of installed middleware is always called on every response.
    Response(Response),
    Ignore,
}

/// Indicates what to do after the `DownloadError` has been processed.
pub enum MiddleWareExceptionResult{
    /// Scrupy will continue processing this exception, executing any other process_exception() methods of installed middleware, until no middleware is left and the default exception handling kicks in.
    Continue,
    /// The process_response() method chain of installed middleware is started, and Scrupy won’t bother calling any other process_exception() methods of middleware.
    Request(Request),
    /// The returned request is rescheduled to be downloaded in the future. This stops the execution of process_exception() methods of the middleware the same as returning a response would.
    Response(Response),
}

/// It processes the request before it's sent to the downloader. Typical
/// application includes setting the timeout and redirection policy.
pub trait DownloaderMiddleware: Send{
    fn process_request(&mut self, request: Request) -> MiddleWareResult;
    fn process_response(&mut self, request_content: &RequestContent, response: Response) -> MiddleWareResult;
    fn process_exception(&mut self, request_content: &RequestContent, error: &DownloadError) -> MiddleWareExceptionResult;
}
