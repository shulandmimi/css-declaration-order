use std::borrow::Cow;

pub enum Sep {
    BlockLeft,
    BlockRight,
    SimpleElement,
    Element,
}

pub trait SepSerialize {
    fn translate(&mut self, sep: Sep) -> Option<String>;
}
