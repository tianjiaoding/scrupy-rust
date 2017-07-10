use spider::Spider;

/// Processed result of itempipeline
pub enum ItemProduct<T>{
    /// An item that will be passed to next ItemPipeline.
    Item(T),
    /// The item is no longer processed by further pipeline components.
    Ignore,
}

pub trait ItemPipeline: Send{
    type ItemType;
    /// Called when an item is processed.
    fn process_item(&mut self, item: Self::ItemType) -> ItemProduct<Self::ItemType>;
    /// Called once when a spider is opened.
    fn open_spider(&mut self, spider: &Box<Spider<ItemType=Self::ItemType>>){
        // Default blank callback.
    }
    /// Called once when a spider is closed.
    fn close_spider(&mut self, spider: &Box<Spider<ItemType=Self::ItemType>>){
        // Default blank callback.
    }
}
