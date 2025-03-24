use crate::math::Transformable;

use super::{Canvas, Contextable, Draw};

mod line;
pub use line::*;

mod background;
pub use background::*;

mod marker;
pub use marker::*;

pub trait DrawCommand {
    type Options: Copy;

    fn draw(self, canvas: &mut Canvas, options: impl Into<Self::Options>);
}

pub struct DrawCommandInstance<D: DrawCommand> {
    command: D,
    options: D::Options,
}

impl<D: DrawCommand> DrawCommandInstance<D> {
    pub fn new(command: D, options: D::Options) -> Self {
        Self { command, options }
    }
}

impl<D: DrawCommand + Transformable + Copy> Draw for DrawCommandInstance<D> {
    fn render(&self, canvas: &mut Canvas) {
        canvas.draw(self.command, self.options);
    }
}