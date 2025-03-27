use crate::{canvas::{color::Color, Canvas}, math::{Transform, Transformable, Vec2}};

use super::Draw;

type MarkerOptions = Color;

pub struct Marker(MarkerOptions);

impl Draw for Marker {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.buffer.fill(self.0.as_u32());
    }
}

impl<T, S> Transformable<T, S> for Marker {
    fn scale(&mut self, _factor: &Vec2<S>) {
        // Do nothing
    }

    fn translate(&mut self, _offset: &Vec2<T>) {
        // Do nothing
    }
}