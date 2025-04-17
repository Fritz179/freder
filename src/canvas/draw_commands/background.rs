use crate::prelude::*;

use super::{Command, DrawCommand};


pub type BackgroundOptions = Color;

pub struct Background;

#[derive(Debug, Clone, Copy)]
pub struct BackgroundCommand(BackgroundOptions);

impl DrawCommand for Background {
    type Command = BackgroundCommand;
    type Options = BackgroundOptions;

    fn into_renderable(self, options: impl Into<Self::Options>) -> Self::Command {
        BackgroundCommand(options.into())
    }
}

impl Command for BackgroundCommand {
    fn render(&mut self, canvas: &mut dyn Image) {
        // let rect = canvas.view_mut().clip;
        let (w, h) = canvas.size_i32();

        for y in 0..h {
            canvas.pixels_mut(0..w, y).color(self.0);
        }
    }
}

impl<T> Transform<T> for BackgroundCommand {
    fn transform(&mut self, _vector: &dyn Transformer<T, 2>) {
        // Do nothing
    }
}