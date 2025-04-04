use crate::{canvas::{color::Color, Canvas}, math::{Line, One, Transform, Transform2D, Vec2, Zero}};

use super::{Command, DrawShape};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LineOptions {
    color: Color,
    pixel_size: Vec2,
}

impl<C: Into<Color>> From<C> for LineOptions {
    fn from(from: C) -> Self {
        Self {
            color: from.into(),
            pixel_size: Vec2::zero()
        }
    }
}

pub trait LineOption: Into<LineOptions> {
    fn middle(self) -> LineOptions {
        let mut options = self.into();
        options.pixel_size = Vec2::one();

        options
    }
}

impl<T: Into<LineOptions>> LineOption for T {}

#[derive(Debug, Clone, Copy)]
pub struct LineCommand {
    line: Line,
    options: LineOptions,
}

impl Transform for LineCommand {
    fn transform(&mut self, transform: &dyn crate::math::Transformer<i32, 2>) {
        self.line.transform(transform);
        self.options.pixel_size.transform(&Transform2D::new_scaling(transform.scaling()));
    }
}

impl DrawShape for Line {
    type Options = LineOptions;
    type Command = LineCommand;

    fn into_renderable(self, options: impl Into<Self::Options>) -> Self::Command {
        LineCommand {  line: self, options: options.into() }
    }
}

impl Command for LineCommand {
    fn render(&mut self, canvas: &mut Canvas) {
        let ((x1, y1), (x2, y2)) = self.line.to_tuple();

        // Center the line to the correct pixel
        let diff = self.options.pixel_size;
        let x1 = x1 + diff.x() / 2;
        let y1 = y1 + diff.y() / 2;
        let x2 = x2 + diff.x() / 2;
        let y2 = y2 + diff.y() / 2;
        
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
    
        let mut x = x1;
        let mut y = y1;
    
        loop {
            if let Some(pixel) = canvas.pixel_mut(x, y) {
                *pixel = self.options.color;
            }

            if x == x2 && y == y2 {
                break
            }
    
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