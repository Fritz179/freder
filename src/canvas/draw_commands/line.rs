use crate::{canvas::{color::Color, Canvas}, math::{Line, Transformable, Vec2}};

use super::{Command, DrawShape, Render};

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

#[derive(Debug, Clone, Copy)]
pub struct LineCommand {
    line: Line,
    options: LineOptions,
}

impl Transformable for LineCommand {
    fn scale(&mut self, factor: f32) {
        self.line.scale(factor);
    }

    fn translate(&mut self, offset: Vec2<i32>) {
        <Line as Transformable>::translate(&mut self.line, offset);
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
    
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
    
        let mut x = x1;
        let mut y = y1;
    
        while x != x2 || y != y2 {
            if let Some(pixel) = canvas.pixel_mut(x, y) {
                *pixel = self.options.color.as_u32();
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