use spider::Spider;

pub enum ItemProduct<T>{
    Item(T),
    Ignore,
}

pub trait ItemPipeline: Send{
    type Items;
    fn process_item(&mut self, item: Self::Items) -> ItemProduct<Self::Items>;
    fn open_spider(&mut self, spider: &Box<Spider<Item=Self::Items>>){

    }
    fn close_spider(&mut self, spider: &Box<Spider<Item=Self::Items>>){

    }
}
