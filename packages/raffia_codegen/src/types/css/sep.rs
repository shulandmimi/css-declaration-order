use crate::sep::{FormatSep, SepRule, SepSerialize};

pub struct CssSep {}

impl SepSerialize<SepRule> for CssSep {
    fn translate(&mut self, sep: SepRule) -> Option<String> {
        match sep {
            SepRule::BlockLeft => Some(String::from("{")),
            SepRule::BlockRight => Some(String::from("}")),
            SepRule::Empty => None,
        }
    }

    fn write_last(&mut self, sep: SepRule) -> Option<String> {
        todo!();
    }

    fn new_line(&mut self, sep: SepRule) -> Option<String> {
        todo!()
    }
}

impl SepSerialize<FormatSep> for CssSep {
    fn translate(&mut self, sep: FormatSep) -> Option<String> {
        // todo!()
        match sep & FormatSep::DELIMITER {
            FormatSep::SPACE => Some(String::from(" ")),
            FormatSep::SEMICOLON => Some(String::from(";")),
            FormatSep::COMMA => Some(String::from(",")),
            _ => None,
        }
    }

    fn write_last(&mut self, sep: FormatSep) -> Option<String> {
        match sep & FormatSep::DELIMITER {
            FormatSep::SEMICOLON => Some(String::from(";")),
            _ => None,
        }
    }

    fn new_line(&mut self, sep: FormatSep) -> Option<String> {
        match sep & FormatSep::LINE {
            FormatSep::MUTIPLE_LINE => Some(String::from("\n")),
            _ => None,
        }
    }
}

impl CssSep {
    pub fn new() -> CssSep {
        CssSep {}
    }
}
