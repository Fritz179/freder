#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color(u32);

impl Color {
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn r(&self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub fn g(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn b(&self) -> u8 {
        self.0 as u8
    }

    pub fn is_transparent(&self) -> bool {
        self.0 & 0xFF_00_00_00 == 0
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Self(color)
    }
}

//                                      A  R  G  B
pub const TRANSPARENT:  Color = Color(0x00_00_00_00);
pub const WHITE:        Color = Color(0xFF_FF_FF_FF);
pub const RED:          Color = Color(0xFF_FF_00_00);
pub const GRAY:         Color = Color(0xFF_33_33_33);
pub const BLACK:        Color = Color(0xFF_00_00_00);
