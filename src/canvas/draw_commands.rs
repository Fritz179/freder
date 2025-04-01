use std::fmt::Debug;

use crate::math::Transformable;

use super::Canvas;

mod line;
pub use line::*;

mod background;
pub use background::*;

mod image;
pub use image::*;

pub trait Render {
    fn render(&self, canvas: &mut Canvas);
}

pub trait Command: Transformable + Debug {
    fn render(&mut self, canvas: &mut Canvas);
}
pub trait CloneCommand: Command {
    fn box_clone(&self) -> Box<dyn CloneCommand>;
}

impl <T: Command + Clone + 'static> CloneCommand for T  {
    fn box_clone(&self) -> Box<dyn CloneCommand> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneCommand> {
    fn clone(&self) -> Box<dyn CloneCommand> {
        self.box_clone()
    }
}

pub trait DrawShape {
    type Options;
    type Command: Command;

    fn into_renderable(self, options: impl Into<Self::Options>) -> Self::Command;
}