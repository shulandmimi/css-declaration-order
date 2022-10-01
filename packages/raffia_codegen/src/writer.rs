pub trait Writer {
    fn write(&mut self);

    fn write_space(&mut self);

    fn write_str(&mut self, v: String) -> crate::Result;

    fn write_raw(&mut self, v: String) -> crate::Result;
}
