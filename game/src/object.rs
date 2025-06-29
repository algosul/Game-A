use std::{any::TypeId, simd::SimdElement};

use dyn_clone::DynClone;

use crate::{
    component::Component,
    transform::{Transform2d, Transform3d},
};
pub mod objects;
pub trait Object: DynClone {
    fn update(&mut self, delta_time: f64);
    fn draw(&self);
    fn get_component_by_id(&self, type_id: TypeId) -> Option<&dyn Component>;
    fn get_mut_component_by_id(&mut self, type_id: TypeId) -> Option<&mut dyn Component>;
}
pub trait Object2d<T: SimdElement, Trans: Transform2d<T>>: Object {
    fn set_transform(&mut self, transform: Trans);
    fn get_transform(&self) -> &Trans;
    fn get_mut_transform(&mut self) -> &mut Trans;
}
pub trait Object3d<T: SimdElement, Trans: Transform3d<T>>: Object {
    fn set_transform(&mut self, transform: Trans);
    fn get_transform(&self) -> &Trans;
    fn get_mut_transform(&mut self) -> &mut Trans;
}
impl dyn Object {
    pub fn get_component<T: Component>(&self) -> Option<&T> {
        self.get_component_by_id(TypeId::of::<T>())?.as_any().downcast_ref::<T>()
    }

    pub fn get_mut_component<T: Component>(&mut self) -> Option<&mut T> {
        self.get_mut_component_by_id(TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<T>()
    }
}
