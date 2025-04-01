use crate::{canvas::{color::Color, Canvas}, math::{Transformable, Vec2}};

use super::Render;

type BackgroundOptions = Color;

pub struct Background(BackgroundOptions);

impl Background {
    pub fn new(color: impl Into<Color>) -> Self {
        Self(color.into())
    }
}

impl Render for Background {
    fn render(&self, canvas: &mut Canvas) {
        let rect = canvas.view.clip;

        for y in rect.y1()..rect.y2() {
            canvas.pixel_slice_mut(rect.x1()..rect.x2(), y).fill(self.0);
        }
    }

}

impl<T: Copy, S: Copy> Transformable<T, S> for Background {
    fn scale(&mut self, _factor:  S) {
        // Do nothing
    }

    fn translate(&mut self, _offset: Vec2<T>) {
        // Do nothing
    }
}