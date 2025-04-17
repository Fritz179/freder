pub mod line;
pub mod background;
pub mod image;
pub mod circle;

use crate::prelude::*;
use std::fmt::Debug;

pub trait Command: Transform + Debug {
    fn render_canvas(&mut self, canvas: &mut dyn Canvas);
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

pub trait DrawCommand {
    type Options;
    type Command: Command;

    fn into_renderable(self, options: impl Into<Self::Options>) -> Self::Command;
}