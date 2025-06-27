use crate::{object::Object, utils::Cloneable};
pub trait Scene: Cloneable {
    fn get_objects(&self) -> &[Box<dyn Object>];
    fn get_mut_objects(&mut self) -> &mut [Box<dyn Object>];
}
