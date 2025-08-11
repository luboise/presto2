pub trait Resize<T> {
    fn set_size<S: Into<T>>(&mut self, new_size: T);
    fn get_size(&self) -> T;
}

pub trait Updates {
    fn update(&mut self);
}
