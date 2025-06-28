use dyn_clone::DynClone;

use crate::object::Object;
pub mod scenes;
pub trait Scene: DynClone {
    fn get_objects<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Object> + 'a>;
    fn get_mut_objects<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = &'a mut (dyn Object + 'static)> + 'a>;
}
