use crate::Writer;
use std::io::Write;

pub struct CssWriter<T>
where
    T: Write,
{
    write: T,
}

impl<T> CssWriter<T>
where
    T: Write,
{
    pub fn new(write: T) -> Self {
        CssWriter { write }
    }
}

impl<T> Writer for CssWriter<T>
where
    T: Write,
{
    fn write(&mut self) {
        todo!()
    }
    fn write_space(&mut self) {
        todo!()
    }

    fn write_str(&mut self, v: String) -> crate::Result {
        todo!()
    }

    fn write_raw(&mut self, v: String) -> crate::Result {
        self.write.write(v.as_bytes()).unwrap();
        Ok(())
    }
}
