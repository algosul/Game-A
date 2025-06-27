use crate::{object::Object, utils::Cloneable};
pub mod scenes;
pub trait Scene: Cloneable {
    fn get_objects(&self) -> Vec<&dyn Object>;
    fn get_mut_objects(&mut self) -> Vec<&mut dyn Object>;
}
