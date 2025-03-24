#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color(u32);

impl Color {
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Self(color)
    }
}

pub const WHITE: Color = Color(0xFFFFFF);
