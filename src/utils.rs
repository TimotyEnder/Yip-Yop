use std::{char::MAX, cmp::max};

use crate::{
    model::elements::pos3::Pos3, screenspace::elements::screenspace_position::ScreenPosition,
};

pub fn highest_z_from_point_list(positions: Vec<Pos3>) -> f64 {
    let mut max_pos: f64 = 0.0;
    for pos in positions.iter() {
        max_pos = max(pos.z() as isize, max_pos as isize) as f64;
    }
    max_pos
}
