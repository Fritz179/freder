use crate::prelude::*;

use super::{Command, DrawCommand};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CircleOption {
    stroke_color: Color,
    fill_color: Color,
}

impl<C: Into<Color>> From<C> for CircleOption {
    fn from(from: C) -> Self {
        Self {
            stroke_color: from.into(),
            fill_color: TRANSPARENT,
        }
    }
}

pub trait CircleOptionTrait: Into<CircleOption> {
    fn fill(self, color: impl Into<Color>) -> CircleOption {
        let mut options = self.into();
        options.fill_color = color.into();

        options
    }
}

impl<T: Into<CircleOption>> CircleOptionTrait for T {}

#[derive(Debug, Clone, Copy)]
pub struct CircleCommand {
    circle: Circle,
    options: CircleOption,
}

impl Transform for CircleCommand {
    fn transform(&mut self, transform: &dyn Transformer<i32, 2>) {
        self.circle.transform(transform);
    }
}

impl DrawCommand for Circle {
    type Options = CircleOption;
    type Command = CircleCommand;

    fn into_renderable(self, options: impl Into<Self::Options>) -> Self::Command {
        CircleCommand {  circle: self, options: options.into() }
    }
}

impl Command for CircleCommand {
    fn render_canvas(&mut self, canvas: &mut dyn Canvas) {
        let (cx, cy) = self.circle.center().to_tuple();
        let mut x = 0;
        let mut y = self.circle.radius();
    
        // Decision parameter.
        let mut p = 1 - self.circle.radius();
        let color = self.options.stroke_color;
        let fill = self.options.fill_color;
    
        // Iterate through the first octant.
        while x <= y {
    
            // Each computed (x,y) produces eight symmetrical points:
            canvas.pixel_mut(cx + x, cy + y).color(color);
            canvas.pixel_mut(cx - x, cy + y).color(color);
            canvas.pixel_mut(cx + x, cy - y).color(color);
            canvas.pixel_mut(cx - x, cy - y).color(color);


            canvas.pixel_mut(cx + y, cy + x).color(color);
            canvas.pixel_mut(cx - y, cy + x).color(color);
            canvas.pixel_mut(cx + y, cy - x).color(color);
            canvas.pixel_mut(cx - y, cy - x).color(color);
    
            if fill != TRANSPARENT {
                // For the horizontal line at cy + y, fill between (cx - x + 1) and (cx + x - 1).
                if x < y { // Only fill if there's an interval.
                    canvas.pixels_mut((cx - x)..(cx + x + 1), cy + y - 1).color(fill);
                    canvas.pixels_mut((cx - x)..(cx + x + 1), cy - y + 1).color(fill);

                    // // Similarly, fill between the horizontal extents for the points when x and y swap.
                    canvas.pixels_mut((cx - y + 1)..(cx + y), cy + x).color(fill);
                    canvas.pixels_mut((cx - y + 1)..(cx + y), cy - x).color(fill);
                }
            }

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