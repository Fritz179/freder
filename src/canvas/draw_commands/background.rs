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
    fn render(&mut self, canvas: &mut Canvas) {
        let rect = canvas.view.clip;

        for y in rect.y1()..rect.y2() {
            canvas.pixel_slice_mut(rect.x1()..rect.x2(), y).fill(self.0);
        }
    }
}

impl<T> Transform<T> for BackgroundCommand {
    fn transform(&mut self, _vector: &dyn Transformer<T, 2>) {
        // Do nothing
    }
}