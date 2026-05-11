use std::collections::HashMap;

use crate::{
    model::elements::pos3::Pos3,
    screenspace::{elements::drawable::Drawable, screen::screen::Screen},
};

pub struct Scene {
    screen: Screen,
    objects: HashMap<Pos3, Box<dyn Drawable>>,
}
impl Scene {
    pub fn with_dimensions(height: &usize, width: &usize) -> Self {
        Self {
            screen: Screen::with_dimensions(width, height),
            objects: HashMap::new(),
        }
    }
    pub fn add_object(&mut self, object: Box<dyn Drawable>) {
        self.objects.insert(object.position(), object);
    }
}
