use crate::model::elements::pos3::Pos3;

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
