pub struct Pos3 {
    x: isize,
    y: isize,
    z: isize,
}
impl Default for Pos3 {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}
impl Pos3 {
    pub fn new(x: &isize, y: &isize, z: &isize) -> Self {
        Self {
            x: *x,
            y: *y,
            z: *z,
        }
    }
}
