pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl Color {
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
}
