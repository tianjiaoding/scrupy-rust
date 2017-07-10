extern crate hyper;
// use self::hyper::client::response::Response;
use spider::Spider;
use downloader::{Request, Response, RequestContent, Method};
use item_pipeline::{ItemPipeline, ItemProduct};
use downloader_middleware::{DownloaderMiddleware, MiddleWareResult, MiddleWareExceptionResult};
use scheduler::Scheduler;
use std::sync::Arc;
use std::thread::spawn;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

/// Result for processing an request after it's sent to downloader and
/// before it goes to item pipeline.
enum IntermediateProcessResult{
    Request(Request),
    Response(Response),
    Ignore,
}

struct FinalProcessResult<ItemType>{
    new_tasks: Vec<Task<ItemType>>,
    worker_id: usize,
}

pub struct Task<ItemType>{
    request: Request,
    crawler: Arc<Box<Crawler<ItemType>>>,
}

pub struct Crawler<ItemType>{
    spider: Box<Spider<ItemType=ItemType>>,
    item_pipelines: Vec<Box<ItemPipeline<ItemType=ItemType>>>,
    downloader_middleware: Vec<Box<DownloaderMiddleware>>,
}

struct Engine<ItemType: 'static>{
    crawlers: Vec<Arc<Box<Crawler<ItemType>>>>,
    scheduler: Scheduler<ItemType>,
    workers: Vec<JoinHandle<()>>,
    /// Sending tasks to each worker.
    txs: Vec<Sender<Task<ItemType>>>,
    /// Receiving results from all the workers.
    rx: Receiver<FinalProcessResult<ItemType>>,
}

impl<ItemType: 'static> Engine<ItemType>{
    fn new(n_workers: usize) -> Engine<ItemType>{
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
            match ip.process_item(item){
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
            match dm.process_response(request_content, response){
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
            match dm.process_request(request){
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
                    match dm.process_exception(&request_content, &e){
                        MiddleWareExceptionResult::Continue => (),
                        MiddleWareExceptionResult::Request(r) => return IntermediateProcessResult::Request(r),
                        MiddleWareExceptionResult::Response(r) => return Self::download_response_chain(&request_content, r, cralwer.clone()),
                    }
                }
                return IntermediateProcessResult::Ignore;
            },
        }
    }
}
