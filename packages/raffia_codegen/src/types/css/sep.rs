use crate::sep::{Sep, SepSerialize};

pub struct CssSep {}

impl SepSerialize for CssSep {
    fn translate(&mut self, sep: Sep) -> Option<String> {
        match sep {
            Sep::BlockLeft => Some(String::from("{")),
            Sep::BlockRight => Some(String::from("}")),
            Sep::SimpleElement => Some(String::from(",")),
            Sep::Element => Some(String::from(",")),
            Sep::SelectorList => Some(String::from(",")),
            Sep::Declarations => Some(String::from(";")),
            Sep::Space => Some(String::from(" ")),
            Sep::Empty => None,
        }
    }

    fn write_last(&mut self, sep: Sep) -> Option<String> {
        match sep {
            Sep::BlockLeft => None,
            Sep::BlockRight => None,
            Sep::SimpleElement => None,
            Sep::Element => None,
            Sep::SelectorList => None,
            Sep::Declarations => Some(String::from(";")),
            Sep::Space => None,
            Sep::Empty => None,
        }
    }
}

impl CssSep {
    pub fn new() -> CssSep {
        CssSep {}
    }
}
