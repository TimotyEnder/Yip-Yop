use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Pos3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Pos3 {
    fn eq(&self, other: &Self) -> bool {
        self.x.to_bits() == other.x.to_bits()
            && self.y.to_bits() == other.y.to_bits()
            && self.z.to_bits() == other.z.to_bits()
    }
}

impl Eq for Pos3 {}

impl Hash for Pos3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}

impl Default for Pos3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
impl Pos3 {
    pub const ZERO: Pos3 = Pos3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn rotate_around_pivot(&mut self, angle_x: f64, angle_y: f64, angle_z: f64, pivot: &Pos3) {
        self.x -= pivot.x;
        self.y -= pivot.y;
        self.z -= pivot.z;
        // Rotate around X axis
        self.rotate_x(angle_x);
        // Rotate around Y axis
        self.rotate_y(angle_y);
        // Rotate around Z axis
        self.rotate_z(angle_z);
        self.x += pivot.x;
        self.y += pivot.y;
        self.z += pivot.z;
    }
    pub fn translate(&mut self, vector: (f64, f64, f64)) {
        self.x += vector.0;
        self.y += vector.1;
        self.z += vector.2;
    }
    fn rotate_x(&mut self, angle: f64) {
        let cos = angle.cos();
        let sin = angle.sin();

        let old_y = self.y;
        self.y = self.y * cos - self.z * sin;
        self.z = old_y * sin + self.z * cos;
    }
    fn rotate_y(&mut self, angle: f64) {
        let cos = angle.cos();
        let sin = angle.sin();

        let old_x = self.x;
        self.x = self.x * cos + self.z * sin;
        self.z = -old_x * sin + self.z * cos;
    }
    fn rotate_z(&mut self, angle: f64) {
        let cos = angle.cos();
        let sin = angle.sin();

        let old_x = self.x;
        self.x = self.x * cos - self.y * sin;
        self.y = old_x * sin + self.y * cos;
    }
}
