use crate::{canvas::{color::Color, Canvas}, math::{Clip, Rect, Transformable, Vec2}};

use super::DrawCommand;

type BackgroundOptions = Color;

pub struct Background;

impl DrawCommand for Background {
    type Options = BackgroundOptions;

    fn draw(self, canvas: &mut Canvas, options: impl Into<Self::Options>) {
        canvas.buffer.fill(options.into().as_u32());
    }
}

impl<T, S> Transformable<T, S> for Background {
    fn scale(&mut self, _factor: Vec2<S>) {
        // Do nothing
    }

    fn translate(&mut self, _offset: Vec2<T>) {
        // Do nothing
    }
}

impl Clip for Background {
    fn clip_rect(self, rect: &Rect) -> Option<Self> {
        Some(self)
    }
}