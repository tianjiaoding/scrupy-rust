use spider::Spider;

pub enum ItemProduct<T>{
    Item(T),
    Ignore,
}

pub trait ItemPipeline: Send + Sync{
    type Items;
    fn process_item(&self, item: Self::Items) -> ItemProduct<Self::Items>;
    fn open_spider(&self, spider: &Box<Spider<Item=Self::Items>>){

    }
    fn close_spider(&self, spider: &Box<Spider<Item=Self::Items>>){

    }
}
