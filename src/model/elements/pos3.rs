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
}
