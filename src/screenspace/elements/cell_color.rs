#[derive(Clone, Default, Copy)]
pub struct CellColor {
    r: u8,
    g: u8,
    b: u8,
}
impl CellColor {
    pub const BLACK: CellColor = CellColor {
        r: 0 as u8,
        b: 0 as u8,
        g: 0 as u8,
    };
    pub const WHITE: CellColor = CellColor {
        r: 255 as u8,
        b: 255 as u8,
        g: 255 as u8,
    };
    pub fn from_rgb(r: &u8, g: &u8, b: &u8) -> Self {
        Self {
            r: *r,
            g: *g,
            b: *b,
        }
    }
    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn ansi_code(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}
