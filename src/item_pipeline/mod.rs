use spider::Spider;

pub enum ItemProduct<T>{
    Item(T),
}

pub trait ItemPipeline{
    type ItemIn;
    type ItemOut;
    fn process_item<T, U: Spider>(&mut self, item: Self::ItemIn, spider: &U) -> ItemProduct<Self::ItemOut>;
    fn open_spider<U: Spider>(&mut self, spider: &U);
    fn close_spider<U: Spider>(&mut self, spider: &U);
}
