pub trait Cloneable {
    fn clone_to_box(&self) -> Box<dyn Cloneable>;
}
impl<T: Clone + 'static> Cloneable for T {
    fn clone_to_box(&self) -> Box<dyn Cloneable> { Box::new(self.clone()) }
}
