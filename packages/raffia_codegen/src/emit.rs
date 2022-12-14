use std::fmt::Result;

pub trait Emit<T> {
    fn emit(&mut self, node: &T) -> Result;
}

impl<T, E> Emit<&'_ T> for E
where
    E: Emit<T>,
{
    #[allow(clippy::only_used_in_recursion)]
    #[inline]
    fn emit(&mut self, node: &&'_ T) -> Result {
        self.emit(&**node)
    }
}

impl<T, E> Emit<Option<T>> for E
where
    E: Emit<T>,
{
    #[inline]
    fn emit(&mut self, node: &Option<T>) -> Result {
        match node {
            Some(node) => self.emit(node),
            None => Ok(()),
        }
    }
}

impl<T, E> Emit<Box<T>> for E
where
    E: Emit<T>,
{
    #[inline]
    fn emit(&mut self, node: &Box<T>) -> Result {
        self.emit(&**node)
    }
}
