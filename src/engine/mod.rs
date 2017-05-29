use spider::Spider;
use item_pipeline::ItemPipeline;

struct Engine<Items>{
    spiders: Vec<Box<Spider<Item=Items>>>,
    // item_pipelines: Vec<Box<ItemPipeline>>,

}

impl<Items> Engine<Items>{
    fn start(&self){
        let mut start_requests: Vec<Request> = vec![];
        for &s in spiders{
            s.
        }
    }
}
