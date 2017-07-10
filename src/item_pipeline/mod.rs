use spider::Spider;

/// Processed result of itempipeline
pub enum ItemProduct<T>{
    /// An item that will be passed to next ItemPipeline. 
    Item(T),
    /// The item is no longer processed by further pipeline components.
    Ignore,
}

pub trait ItemPipeline: Send + Sync{
    type ItemType;
    fn process_item(&self, item: Self::ItemType) -> ItemProduct<Self::ItemType>;
    fn open_spider(&self, spider: &Box<Spider<ItemType=Self::ItemType>>){
        // Default implementation of opening
    }
    fn close_spider(&self, spider: &Box<Spider<ItemType=Self::ItemType>>){

    }
}
