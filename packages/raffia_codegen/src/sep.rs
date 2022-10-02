use bitflags::bitflags;

#[derive(Clone, Copy, Debug)]
pub enum SepRule {
    BlockLeft,
    BlockRight,
    Empty,
}
bitflags! {
    pub struct FormatSep: u32 {
        // none
        const NONE = 0;

        // line
        const SINGLE_LINE = 1 << 0;
        const MUTIPLE_LINE = 1 << 1;
        const LINE = Self::SINGLE_LINE.bits | Self::MUTIPLE_LINE.bits;

        // Delimiters
        /// " "
        const SPACE = 1 << 2;
        /// ;
        const SEMICOLON = 1 << 3;
        /// ,
        const COMMA = 1 << 4;
        const DELIMITER = Self::SPACE.bits | Self::SEMICOLON.bits | Self::COMMA.bits;
    }
}

pub trait SepSerialize<T> {
    fn translate(&mut self, sep: T) -> Option<String>;
    fn write_last(&mut self, sep: T) -> Option<String>;
    fn new_line(&mut self, sep: T) -> Option<String>;
}
