use crate::model::elements::pos3::Pos3;
#[derive(Hash, Clone, Copy, Eq)]
pub struct ScreenPosition {
    x: usize,
    y: usize,
}
impl ScreenPosition {
    pub fn with_pos(x: &usize, y: &usize) -> Self {
        Self { x: *x, y: *y }
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
}
impl PartialEq for ScreenPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
