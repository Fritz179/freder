mod vec2;
pub use vec2::Vec2;

mod line;
pub use line::Line;

mod rect;
pub use rect::Rect;

mod numbers;
pub use numbers::*;

mod collisions;
pub use collisions::*;

pub trait Transformable<T = i32, S = f32> {
    fn transform(&mut self, transform: &Transform<T, S>) {
        self.scale(&transform.scaling);
        self.translate(&transform.translation);
    }

    fn translate(&mut self, offset: &Vec2<T>);
    fn scale(&mut self, factor: &Vec2<S>);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transform<T = i32, S = f32>{
    scaling: Vec2<S>,
    translation: Vec2<T>,
}

impl<T, S> Transform<T, S> {
    pub fn new(translation: Vec2<T>, scaling: Vec2<S>) -> Self {
        Self { translation, scaling }
    }

    pub fn translation(&self) -> &Vec2<T> {
        &self.translation
    }

    pub fn translation_mut(&mut self) -> &mut Vec2<T> {
        &mut self.translation
    }

    pub fn set_translation(&mut self, translation: Vec2<T>) {
        self.translation = translation;
    }

    pub fn scaling(&self) -> &Vec2<S> {
        &self.scaling
    }

    pub fn scaling_mut(&mut self) -> &mut Vec2<S> {
        &mut self.scaling
    }

    pub fn set_scaling(&mut self, scaling: Vec2<S>) {
        self.scaling = scaling;
    }
}


impl<T: Zero, S: One> Transform<T, S> {
    pub fn identity() -> Self {
        Self {
            translation: Vec2::zero(),
            scaling: Vec2::one(),
        }
    }
}

impl<T: Zero, S: One> Default for Transform<T, S> {
    fn default() -> Self {
        Self::identity()
    }
}