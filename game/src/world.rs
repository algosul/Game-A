use std::sync::{Arc, Mutex};

use crate::scene::Scene;
pub struct World {
    scenes: Vec<Arc<Mutex<dyn Scene + Send>>>,
}
impl Default for World {
    fn default() -> Self { Self::new() }
}
impl World {
    pub fn new() -> Self { World { scenes: Vec::new() } }

    pub fn update(&self) {
        for scene in &self.scenes {
            scene.lock().unwrap().update(0.0);
        }
    }

    pub fn draw(&self) {
        for scene in &self.scenes {
            scene.lock().unwrap().draw();
        }
    }

    pub fn load_scene(&mut self, scene: Arc<Mutex<dyn Scene + Send>>) {
        self.scenes.push(scene);
    }

    pub fn remove_scene(&mut self, index: usize) { self.scenes.remove(index); }
}
