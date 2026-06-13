use std::collections::HashMap;

use winit::keyboard::KeyCode::SuperLeft;

use crate::{
    model::elements::pos3::Pos3,
    screenspace::elements::{cell_color::CellColor, screenspace_position::ScreenPosition},
};

pub struct Screen {
    current_frame_pixel_buffer: Vec<u8>,
    height: usize,
    width: usize,
    camera_translations: HashMap<usize, ((Pos3, (f64, f64, f64)), f64)>,
}

impl Screen {
    pub fn add_camera_translation(&mut self, translation: (usize, (Pos3, (f64, f64, f64)), f64)) {
        self.camera_translations
            .insert(translation.0, (translation.1, translation.2));
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn with_dimensions(height: usize, width: usize) -> Self {
        Self {
            current_frame_pixel_buffer: vec![0u8; width * height * 4],
            width,
            height,
            camera_translations: HashMap::new(),
        }
    }
    pub fn draw_and_flush_list(&mut self, buffer: &mut [u8]) {
        buffer.copy_from_slice(&self.current_frame_pixel_buffer);
        self.current_frame_pixel_buffer = vec![0u8; self.width * self.height * 4];
    }
    pub fn color_cell(&mut self, pos: &ScreenPosition, color: &CellColor) {
        let idx = (pos.y * self.width + pos.x) * 4;
        if (1..self.height - 1).contains(&pos.y) && (1..self.width - 2).contains(&pos.x) {
            self.current_frame_pixel_buffer[idx] = color.r();
            self.current_frame_pixel_buffer[idx + 1] = color.g();
            self.current_frame_pixel_buffer[idx + 2] = color.b();
            self.current_frame_pixel_buffer[idx + 3] = 255;
        }
    }
    pub fn camera_depth(&self, value: &Pos3) -> f64 {
        let (pos, rot) = self
            .camera_translations
            .iter()
            .nth(0)
            .expect("No cameras present in the scene")
            .1
            .0;
        let mut camera_pos = Pos3::new(value.x - pos.x, value.y - pos.y, value.z - pos.z);
        camera_pos.rotate_around_pivot(-rot.0, -rot.1, -rot.2, &Pos3::new(0.0, 0.0, 0.0));
        camera_pos.z
    }
    pub fn pos3_is_visible(&self, value: &Pos3) -> bool {
        let ((camera_pos, camera_rot), fov_y) = self
            .camera_translations
            .iter()
            .nth(0)
            .expect("No cameras present in the scene")
            .1;
        let mut camera_value = Pos3::new(
            value.x - camera_pos.x,
            value.y - camera_pos.y,
            value.z - camera_pos.z,
        );
        let pivot = Pos3::new(0.0, 0.0, 0.0);
        camera_value.rotate_around_pivot(-camera_rot.0, -camera_rot.1, -camera_rot.2, &pivot);

        if camera_value.z <= 0.0 {
            return false;
        }

        let half_height_tan = (fov_y / 2.0).tan();
        let half_width_tan = half_height_tan * (self.width as f64 / self.height as f64);

        camera_value.y.abs() <= camera_value.z * half_height_tan
            && camera_value.x.abs() <= camera_value.z * half_width_tan
    }
    pub fn project_point(&self, value: &Pos3) -> ScreenPosition {
        let (camera_pos, camera_rot) = self
            .camera_translations
            .iter()
            .nth(0)
            .expect("No cameras present in the scene")
            .1
            .0;
        let mut camera_value = Pos3::new(
            value.x - camera_pos.x,
            value.y - camera_pos.y,
            value.z - camera_pos.z,
        );
        let pivot = Pos3::new(0.0, 0.0, 0.0);
        camera_value.rotate_around_pivot(-camera_rot.0, -camera_rot.1, -camera_rot.2, &pivot);

        let x = camera_value.x;
        let y = camera_value.y;
        let z = camera_value.z;

        if z <= 0.0 {
            return ScreenPosition::with_pos(0, 0);
        }

        let screen_x = ((x / z) + 1.0) / 2.0 * (self.width as f64);
        let screen_y = ((y / z) + 1.0) / 2.0 * (self.height as f64);
        let screen_y = self.height as f64 - screen_y;

        let screen_x = screen_x.clamp(0.0, (self.width - 1) as f64);
        let screen_y = screen_y.clamp(0.0, (self.height - 1) as f64);
        ScreenPosition::with_pos(screen_x as usize, screen_y as usize)
    }
}
