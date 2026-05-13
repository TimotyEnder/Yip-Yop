use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Pos3 {
    x: f64,
    y: f64,
    z: f64,
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
    pub fn new(x: &f64, y: &f64, z: &f64) -> Self {
        Self {
            x: *x,
            y: *y,
            z: *z,
        }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn rotate(&mut self, angle_x: f64, angle_y: f64, angle_z: f64) {
        // Rotate around X axis
        self.rotate_x(angle_x);

        // Rotate around Y axis
        self.rotate_y(angle_y);

        // Rotate around Z axis
        self.rotate_z(angle_z);
    }
    fn rotate_x(&self, angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Self {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }

    // Rotate around Y axis (Yaw)
    fn rotate_y(&self, angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Self {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }

    // Rotate around Z axis (Roll)
    fn rotate_z(&self, angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
        }
    }
}
