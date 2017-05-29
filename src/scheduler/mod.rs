use http::{Request, Method};

pub struct Scheduler{
    pub queue: Vec<Request>,
}

impl Scheduler{
    pub fn enqueue(&mut self, request: Request){
        self.queue.push(request);
    }
    pub fn dequeue(&mut self) -> Option<Request>{
        self.queue.pop()
    }
}
