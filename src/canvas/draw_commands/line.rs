use crate::{canvas::{color::Color, Canvas}, math::Line};

use super::DrawCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineOptions {
    color: Color,
    thickness: u32,
}

impl<C: Into<Color>> From<C> for LineOptions {
    fn from(from: C) -> Self {
        Self {
            color: from.into(),
            thickness: 1,
        }
    }
}

pub trait LineOption: Into<LineOptions> {
    fn width(self, width: u32) -> LineOptions {
        let mut options = self.into();
        options.thickness = width;

        options
    }
}

impl<T: Into<LineOptions>> LineOption for T {}


impl DrawCommand for Line {
    type Options = LineOptions;

    fn draw(self, canvas: &mut Canvas, options: impl Into<Self::Options>) {
        let options = options.into();

        let ((x1, y1), (x2, y2)) = self.to_tuple();
    
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
    
        let mut x = x1;
        let mut y = y1;
    
        while x != x2 || y != y2 {
            let p = x + y * canvas.width as i32;
            canvas.buffer[p as usize] = options.color.as_u32();
    
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}