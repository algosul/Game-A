use marco::Scene;

use crate::{
    object::{objects::Button, Object},
    scene::Scene,
};
#[derive(Scene)]
pub struct DynamicScene {
    #[scene(objects)]
    objects: Vec<Box<dyn Object + Send>>,
}
impl Clone for DynamicScene {
    fn clone(&self) -> Self {
        Self {
            objects: self
                .objects
                .iter()
                .map(|object| dyn_clone::clone_box(object.as_ref()))
                .collect(),
        }
    }
}
#[derive(Clone, Scene)]
pub struct StartMenuScene {
    #[scene(object)]
    play_button: Button,
}
impl Default for DynamicScene {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicScene {
    pub fn new() -> Self { Self { objects: Vec::new() } }
}
