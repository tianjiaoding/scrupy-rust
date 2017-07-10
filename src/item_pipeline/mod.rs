use spider::Spider;

/// Processed result of itempipeline
pub enum ItemProduct<T>{
    /// An item that will be passed to next ItemPipeline.
    Item(T),
    /// The item is no longer processed by further pipeline components.
    Ignore,
}

pub trait ItemPipeline: Send{
    type Items;
    fn process_item(&mut self, item: Self::Items) -> ItemProduct<Self::Items>;
    fn open_spider(&mut self, spider: &Box<Spider<ItemType=Self::Items>>){

    }
    fn close_spider(&mut self, spider: &Box<Spider<ItemType=Self::Items>>){

    }
}
