struct ComponentList<T: Sized> {
    data: Vec<T>,
}

impl<T> ComponentList<T> {
    pub fn new(initial_size: usize) -> ComponentList<T> {
        let data = Vec::with_capacity(initial_size);
        ComponentList::<T> { data }
    }

    // pub fn add(component: T) -> usize {}
}
