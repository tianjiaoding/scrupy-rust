use downloader::{Request, Method};
use engine::Crawler;
use std::sync::Arc;
use engine::Task;

pub struct Scheduler<ItemType>{
    pub queue: Vec<Task<ItemType>>,
}

impl<ItemType> Scheduler<ItemType>{
    pub fn new() -> Self{
        Scheduler{
            queue: vec![],
        }
    }
    pub fn enqueue(&mut self, task: Task<ItemType>){
        self.queue.push(task);
    }
    pub fn dequeue(&mut self) -> Option<Task<ItemType>>{
        self.queue.pop()
    }
}
