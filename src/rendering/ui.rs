use super::*;

pub struct RadioButton<'a> {
    pub radio_pos: (f32, f32),
    pub label_pos: (f32, f32),
    pub label_image: &'a graphics::Image,
}

impl<'a> RadioButton<'a> {
    pub fn new(
        radio_pos: (f32, f32),
        label_pos: (f32, f32),
        label_image: &'a graphics::Image,
    ) -> Self {
        Self {
            radio_pos,
            label_pos,
            label_image,
        }
    }
}

pub struct SceneTransformation {
    pub scene_scale: (f32, f32),
    pub translation: (f32, f32),
}

impl SceneTransformation {
    pub fn new(scene_scale: (f32, f32), translation: (f32, f32)) -> Self {
        Self {
            scene_scale,
            translation,
        }
    }
}
