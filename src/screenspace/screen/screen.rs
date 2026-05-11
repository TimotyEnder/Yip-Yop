use std::collections::HashMap;

use crate::screenspace::elements::{
    color::Color, screenspace_object::ScreenspaceObject, screenspace_position::ScreenPos,
};
pub struct Screen {
    screen_vec: Vec<Vec<Color>>,
    objects: HashMap<ScreenPos, Box<dyn ScreenspaceObject>>,
}
