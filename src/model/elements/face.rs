use crate::screenspace::elements::cell_color::CellColor;

#[derive(Copy, Clone)]
pub struct Face {
    pub indices: (usize, usize, usize),
    pub color: Option<CellColor>,
}
