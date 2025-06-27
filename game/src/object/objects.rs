use std::any::{Any, TypeId};
use crate::component::Component;
use crate::object::Object;

#[derive(Clone, Debug)]
pub struct Button {}
impl Object for Button {

    fn update(&mut self, delta_time: f64) { todo!() }

    fn draw(&self) { todo!() }

    fn get_component_by_id(&self, type_id: TypeId) -> Option<&dyn Component> {
        todo!()
    }

    fn get_mut_component_by_id(&mut self, type_id: TypeId) -> Option<&mut dyn Component> {
        todo!()
    }
}