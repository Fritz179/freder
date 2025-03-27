use crate::{canvas::{color::Color, Canvas}, math::{Clip, Rect, Transform, Transformable, Vec2}};

use super::Draw;

type BackgroundOptions = Color;

pub struct Background(BackgroundOptions);

impl Background {
    pub fn new(color: impl Into<Color>) -> Self {
        Self(color.into())
    }
}

impl Draw for Background {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.buffer.fill(self.0.as_u32());
    }

}

impl<T, S> Transformable<T, S> for Background {
    fn scale(&mut self, _factor:  &Vec2<S>) {
        // Do nothing
    }

    fn translate(&mut self, _offset: &Vec2<T>) {
        // Do nothing
    }
}

impl Clip for Background {
    fn clip_rect(self, rect: &Rect) -> Option<Self> {
        Some(self)
    }
}