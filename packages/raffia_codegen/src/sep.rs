#[derive(Clone, Copy)]
pub enum Sep {
    BlockLeft,
    BlockRight,
    SimpleElement,
    Element,
    Empty,
}

pub trait SepSerialize {
    fn translate(&mut self, sep: Sep) -> Option<String>;
}
