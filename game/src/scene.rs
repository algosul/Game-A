use dyn_clone::DynClone;

use crate::object::Object;
pub mod scenes;
pub trait Scene: DynClone {
    fn update(&mut self, delta_time: f64);
    fn draw(&self);
    fn get_objects<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a (dyn Object + Send + 'a)> + '_>;
    fn get_mut_objects<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = &'a mut (dyn Object + Send + 'a)> + '_>;
}
