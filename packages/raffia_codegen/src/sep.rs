#[derive(Clone, Copy, Debug)]
pub enum Sep {
    BlockLeft,
    BlockRight,
    SimpleElement,
    Element,
    SelectorList,
    Declarations,
    Space,
    Empty,
}

pub trait SepSerialize {
    fn translate(&mut self, sep: Sep) -> Option<String>;
    fn write_last(&mut self, sep: Sep) -> Option<String>;
}
