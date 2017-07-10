//! Engine module

extern crate hyper;
use spider::Spider;
use downloader::{Request, Response, RequestContent};
use item_pipeline::{ItemPipeline, ItemProduct};
use downloader_middleware::{DownloaderMiddleware, MiddleWareResult, MiddleWareExceptionResult};
use scheduler::Scheduler;
use std::sync::Arc;
use std::thread::spawn;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Mutex;

/// Result for processing an request after it's sent to downloader and
/// before it goes to item pipeline.
enum IntermediateProcessResult{
    Request(Request),
    Response(Response),
    Ignore,
}

/// New tasks and the worker that produce the new task.
struct FinalProcessResult<ItemType>{
    new_tasks: Vec<Task<ItemType>>,
    worker_id: usize,
}

/// A task sent to worker, including a request and the crawler related to this crawler.
pub struct Task<ItemType>{
    /// The request
    request: Request,
    /// The crawler that issues the request
    crawler: Arc<Box<Crawler<ItemType>>>,
}

/// A craweler includes one spider, item_pipelines and downloader middlewares.
pub struct Crawler<ItemType>{
    spider: Box<Spider<ItemType=ItemType>>,
    item_pipelines: Vec<Mutex<Box<ItemPipeline<ItemType=ItemType>>>>,
    downloader_middleware: Vec<Mutex<Box<DownloaderMiddleware>>>,
}

/// The core engine that controls the top control flow.
pub struct Engine<ItemType: 'static>{
    /// A set of crawlers that the engine handles.
    crawlers: Vec<Arc<Box<Crawler<ItemType>>>>,
    /// One scheduler that schedules the tasks based on priority.
    scheduler: Scheduler<ItemType>,
    /// Handles of all the threads.
    workers: Vec<JoinHandle<()>>,
    /// Sending tasks to each worker.
    txs: Vec<Sender<Task<ItemType>>>,
    /// Receiving results from all the workers.
    rx: Receiver<FinalProcessResult<ItemType>>,
}

impl<ItemType: 'static> Engine<ItemType>{
    /// Create a new engine with numbers of workers specified and no crawler.
    pub fn new(n_workers: usize) -> Engine<ItemType>{
        let mut tx_vec = vec![];
        let (id_tx, id_rx) = channel();
        let mut workers = vec![];
        for i in 0..n_workers {
            let (tx, rx) = channel();
            tx_vec.push(tx);
            let id_tx_ = id_tx.clone();
            workers.push(spawn(move || Self::worker(i, rx, id_tx_)));
        }
        Engine{
            crawlers: vec![],
            scheduler: Scheduler::new(),
            workers: workers,
            txs: tx_vec,
            rx: id_rx,
        }
    }
    /// Add a crawler to the engine.
    pub fn add_crawler(&mut self, crawler: Crawler<ItemType>){
        self.crawlers.push(Arc::new(Box::new(crawler)));
    }
    fn init_requests(&mut self){
        for s in &self.crawlers{
            for r in s.spider.start_requests() {
                self.scheduler.enqueue(Task{
                    request: r,
                    crawler: s.clone(),
                });
            }
        }
    }
    /// Run the engine.
    pub fn run(&mut self) {
        // Init all requests
        self.init_requests();
        // Sends one URL to each worker.
        for i in 0..self.workers.len() {
            if !self.send_one(i) {
                break;
            }
        }

        // When a worker is free, sends a new URL to it.
        while let Ok(process_result) = self.rx.recv() {
            for task in process_result.new_tasks{
                self.scheduler.enqueue(task);
            }
            if !self.send_one(process_result.worker_id) {
                break;
            }
        }

        // Closes all the `Sender`s, so `Receiver.recv()` in `worker()`
        // will return `Err`, and then `worker()`'s thread will terminate.
        while let Some(tx_) = self.txs.pop() {
            // Nothing to do. Will call `drop()` when `tx_` goes out of scope.
        }

        // Waits for all workers to finish.
        while let Some(worker) = self.workers.pop() {
            worker.join().unwrap();
        }
    }
    fn send_one(&mut self, i: usize) -> bool {
        match self.scheduler.dequeue() {
            Some(task) => {
                self.txs[i].send(task).unwrap();
                true
            },
            None => false,
        }
    }
    fn worker(id: usize, rx: Receiver<Task<ItemType>>, tx: Sender<FinalProcessResult<ItemType>>){
        while let Ok(task) = rx.recv() {
            let crawler = task.crawler.clone();
            let pr = Self::process_one(task);
            let mut tasks = vec![];
            for r in pr{
                tasks.push(Task{
                    request: r,
                    crawler: crawler.clone(),
                })
            }
            tx.send(FinalProcessResult{
                new_tasks: tasks,
                worker_id: id,
            }).unwrap();
        }
    }

    fn process_one(task: Task<ItemType>) -> Vec<Request>{
        let crawler = task.crawler.clone();
        match Self::download_one(task){
            IntermediateProcessResult::Request(r) => {
                vec![r]
            },
            IntermediateProcessResult::Response(r) => {
                let (requests, ItemType) = crawler.spider.parse(r);
                for item in ItemType{
                    Self::item_process_chain(item, crawler.clone());
                }
                requests
            },
            IntermediateProcessResult::Ignore => {
                vec![]
            }
        }
    }
    fn item_process_chain(item: ItemType, cralwer: Arc<Box<Crawler<ItemType>>>){
        let mut item = item;
        for ip in &cralwer.item_pipelines{
            let mut ip_ = ip.lock().unwrap();
            match ip_.process_item(item){
                ItemProduct::Item(i) => item = i,
                ItemProduct::Ignore => break,
            }
        }
    }
    fn download_response_chain(request_content: &RequestContent,
                                response: Response,
                                cralwer: Arc<Box<Crawler<ItemType>>>)
                                -> IntermediateProcessResult{
        let mut response = response;
        for dm in &cralwer.downloader_middleware{
            let mut dm_ = dm.lock().unwrap();
            match dm_.process_response(request_content, response){
                MiddleWareResult::FinalRequest(r) |
                MiddleWareResult::IntermediateRequest(r) => return IntermediateProcessResult::Request(r),
                MiddleWareResult::Response(r) => response = r,
                MiddleWareResult::Ignore => return IntermediateProcessResult::Ignore,
            }
        }
        return IntermediateProcessResult::Response(response);
    }
    fn download_one(task: Task<ItemType>) -> IntermediateProcessResult{
        let mut request = task.request;
        let mut cralwer = task.crawler;
        for dm in &cralwer.downloader_middleware{
            let mut dm_ = dm.lock().unwrap();
            match dm_.process_request(request){
                MiddleWareResult::FinalRequest(r) => return IntermediateProcessResult::Request(r),
                MiddleWareResult::Response(r) => return IntermediateProcessResult::Response(r),
                MiddleWareResult::IntermediateRequest(r) => request = r,
                MiddleWareResult::Ignore => return IntermediateProcessResult::Ignore,
            }
        }
        let request_content = request.content.clone();
        match request.download(){
            Ok(response) =>{
                return Self::download_response_chain(&request_content,
                    response, cralwer.clone())
            },
            Err(e) => {
                for dm in &cralwer.downloader_middleware{
                    let mut reprocess_response = None; // Using nested process and holding the response in the outer scope avoids acquiring another lock before releasing the current one.
                    {
                        let mut dm_ = dm.lock().unwrap();
                        match dm_.process_exception(&request_content, &e){
                            MiddleWareExceptionResult::Continue => (),
                            MiddleWareExceptionResult::Request(r) => return IntermediateProcessResult::Request(r),
                            MiddleWareExceptionResult::Response(r) => reprocess_response = Some(r),
                        }
                    }
                    if let Some(r) = reprocess_response{
                        return Self::download_response_chain(&request_content, r, cralwer.clone());
                    }
                }
                return IntermediateProcessResult::Ignore;
            },
        }
    }
}
