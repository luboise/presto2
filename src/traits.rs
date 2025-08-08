pub(crate) use crate::types::Size2D;

pub trait Resize {
    fn set_size<S: Into<Size2D>>(&mut self, new_size: Size2D);
    fn get_size(&self) -> Size2D;
}
