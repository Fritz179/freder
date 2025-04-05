use crate::prelude::*;

use super::{Command, DrawCommand};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CircleOptions {
    color: Color,
}

impl<C: Into<Color>> From<C> for CircleOptions {
    fn from(from: C) -> Self {
        Self {
            color: from.into(),
        }
    }
}

pub trait CircleOption: Into<CircleOptions> {
    
}

impl<T: Into<CircleOptions>> CircleOption for T {}

#[derive(Debug, Clone, Copy)]
pub struct CircleCommand {
    circle: Circle,
    options: CircleOptions,
}

impl Transform for CircleCommand {
    fn transform(&mut self, transform: &dyn Transformer<i32, 2>) {
        self.circle.transform(transform);
    }
}

impl DrawCommand for Circle {
    type Options = CircleOptions;
    type Command = CircleCommand;

    fn into_renderable(self, options: impl Into<Self::Options>) -> Self::Command {
        CircleCommand {  circle: self, options: options.into() }
    }
}

impl Command for CircleCommand {
    fn render(&mut self, canvas: &mut Canvas) {
        let (cx, cy) = self.circle.center().to_tuple();
        let mut x = 0;
        let mut y = self.circle.radius();
    
        // Decision parameter.
        let mut p = 1 - self.circle.radius();
        let color = RED;
    
        // Iterate through the first octant.
        while x <= y {
    
            // Each computed (x,y) produces eight symmetrical points:
            canvas.pixel_mut(cx + x, cy + y).map(|c| *c = color);
            canvas.pixel_mut(cx - x, cy + y).map(|c| *c = color);
            canvas.pixel_mut(cx + x, cy - y).map(|c| *c = color);
            canvas.pixel_mut(cx - x, cy - y).map(|c| *c = color);
            canvas.pixel_mut(cx + y, cy + x).map(|c| *c = color);
            canvas.pixel_mut(cx - y, cy + x).map(|c| *c = color);
            canvas.pixel_mut(cx + y, cy - x).map(|c| *c = color);
            canvas.pixel_mut(cx - y, cy - x).map(|c| *c = color);
    
            // Update decision parameter and coordinates.
            if p < 0 {
                p += 2 * x + 3;
            } else {
                p += 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }
    }
}