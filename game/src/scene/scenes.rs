use marco::Scene;

use crate::{
    object::{objects::Button, Object},
    scene::Scene,
};
#[derive(Clone, Scene)]
pub struct StartMenuScene {
    #[scene(object)]
    play_button: Button,
}
