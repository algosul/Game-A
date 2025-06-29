use std::any::TypeId;

use crate::{component::Component, object::Object};
#[derive(Clone, Debug)]
pub struct Button {}
impl Object for Button {
    fn update(&mut self, _delta_time: f64) { todo!() }

    fn draw(&self) { todo!() }

    fn get_component_by_id(&self, _type_id: TypeId) -> Option<&dyn Component> { todo!() }

    fn get_mut_component_by_id(&mut self, _type_id: TypeId) -> Option<&mut dyn Component> {
        todo!()
    }
}
