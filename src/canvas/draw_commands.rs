use crate::math::{Line, Transform2D, Transformable};

use super::{Canvas};

mod line;
pub use line::*;

mod background;
pub use background::*;

mod marker;
pub use marker::*;

pub trait Draw {
    fn draw(&mut self, canvas: &mut Canvas);
}

pub trait DrawShape {
    type Options;
    type Command: Draw + Transformable;

    fn new_command(self, options: impl Into<Self::Options>) -> Self::Command;
}