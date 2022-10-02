use crate::sep::{Sep, SepSerialize};

pub struct CssSep {}

impl SepSerialize for CssSep {
    fn translate(&mut self, sep: Sep) -> Option<String> {
        match sep {
            Sep::BlockLeft => Some(String::from("{")),
            Sep::BlockRight => Some(String::from("}")),
            Sep::SimpleElement => Some(String::from(",")),
            Sep::Element => Some(String::from(",")),
        }
    }
}

impl CssSep {
    pub fn new() -> CssSep {
        CssSep {  }
    }
}
