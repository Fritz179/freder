use crate::{canvas::{color::Color, Canvas}, math::{Transformable, Vec2}};

use super::DrawCommand;

type MarkerOptions = Color;

pub struct Marker;

impl DrawCommand for Marker {
    type Options = MarkerOptions;

    fn draw(self, canvas: &mut Canvas, options: impl Into<Self::Options>) {
        canvas.buffer.fill(options.into().as_u32());
    }
}

impl<T, S> Transformable<T, S> for Marker {
    fn scale(&mut self, _factor: Vec2<S>) {
        // Do nothing
    }

    fn translate(&mut self, _offset: Vec2<T>) {
        // Do nothing
    }
}