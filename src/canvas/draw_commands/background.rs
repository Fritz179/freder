use crate::{canvas::{color::Color, Canvas}, math::Transform};

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

impl<T> Transform<T> for Background {
    fn transform(&mut self, _vector: &dyn crate::math::Transformer<T, 2>) {
        // Do nothing
    }
}