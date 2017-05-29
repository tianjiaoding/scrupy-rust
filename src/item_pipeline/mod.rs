// use spider::Spider;

pub enum ItemProduct<T>{
    Item(T),
}

pub trait ItemPipeline{
    type ItemIn;
    type ItemOut;
    type Spider;
    fn process_item<T>(&mut self, item: Self::ItemIn, spider: &Self::Spider) -> ItemProduct<Self::ItemOut>;
    fn open_spider(&mut self, spider: &Self::Spider){

    }
    fn close_spider(&mut self, spider: &Self::Spider){

    }
}
