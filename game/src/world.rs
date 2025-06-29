use std::{
    sync::{Arc, Mutex},
    thread::sleep_until,
    time::{Duration, Instant},
};

use log::info;

use crate::scene::Scene;
pub struct World {
    scenes:      Vec<Arc<Mutex<dyn Scene + Send>>>,
    last_update: Mutex<Instant>,
    last_draw:   Mutex<Instant>,
}
impl Default for World {
    fn default() -> Self { Self::new() }
}
impl World {
    pub fn new() -> Self {
        World {
            scenes:      Vec::new(),
            last_update: Mutex::new(Instant::now()),
            last_draw:   Mutex::new(Instant::now()),
        }
    }

    pub fn update(&self) {
        let now = Instant::now();
        let until;
        let delta_time;
        {
            let mut last_update = self.last_update.lock().unwrap();
            until = *last_update + Duration::from_secs_f64(1.0 / 60.0);
            delta_time = (now - *last_update).as_secs_f64();
            *last_update = now;
        }
        for scene in &self.scenes {
            scene.lock().unwrap().update(delta_time);
        }
        info!("tick {delta_time}");
        sleep_until(until);
    }

    pub fn draw(&self) {
        let now = Instant::now();
        let until;
        let delta_time;
        {
            let mut last_draw = self.last_draw.lock().unwrap();
            until = *last_draw + Duration::from_secs_f64(1.0 / 60.0);
            delta_time = (now - *last_draw).as_secs_f64();
            *last_draw = now;
        }
        for scene in &self.scenes {
            scene.lock().unwrap().draw();
        }
        sleep_until(until);
    }

    pub fn load_scene(&mut self, scene: Arc<Mutex<dyn Scene + Send>>) {
        self.scenes.push(scene);
    }

    pub fn remove_scene(&mut self, index: usize) { self.scenes.remove(index); }
}
