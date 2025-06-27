use std::any::Any;
pub  mod components;
pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn update(&mut self, delta_time: f64);
    fn draw(&self);
}
