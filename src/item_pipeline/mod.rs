// use spider::Spider;

pub enum ItemProduct<T>{
    Item(T),
}

pub trait ItemPipeline{
    type Items;
    type Spider;
    fn process_item<T>(&mut self, item: Self::Items, spider: &Self::Spider) -> ItemProduct<Self::Items>;
    fn open_spider(&mut self, spider: &Self::Spider){

    }
    fn close_spider(&mut self, spider: &Self::Spider){

    }
}
