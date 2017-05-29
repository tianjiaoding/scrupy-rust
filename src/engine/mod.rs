use spider::Spider;
use http::{Request, Method};
use item_pipeline::ItemPipeline;
use scheduler::Scheduler;

struct Engine<Items>{
    spiders: Vec<Box<Spider<Item=Items>>>,
    // item_pipelines: Vec<Box<ItemPipeline>>,
    scheduler: Scheduler,
}

impl<Items> Engine<Items>{
    fn init_requests(&mut self){
        let mut start_requests: Vec<Request> = vec![];
        // 1: Get initial requests from spiders
        for s in &self.spiders{
            start_requests.append(&mut s.start_requests());
        }
        // 2: Pass initial requests to scheduler
        for r in start_requests{
            self.scheduler.enqueue(r);
        }
    }
    fn worker(request: Request){

    }
}
