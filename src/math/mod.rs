mod vector;
pub use vector::Vec2;

mod line;
pub use line::Line;

mod rect;
pub use rect::Rect;

mod numbers;
pub use numbers::*;

mod collisions;
pub use collisions::*;

pub trait Transformable<T: Copy = i32, S: Copy = f32> {
    fn transform(&mut self, transform: &Transform2D<T, S>) {
        self.scale(transform.scaling);
        self.translate(transform.translation);
    }

    fn translate(&mut self, offset: Vec2<T>);
    fn scale(&mut self, factor: S);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transform2D<T = i32, S = f32>{
    scaling: S,
    translation: Vec2<T>,
}

impl<T, S> Transform2D<T, S> {
    pub fn new(translation: Vec2<T>, scaling: S) -> Self {
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

    pub fn scaling(&self) -> &S {
        &self.scaling
    }

    pub fn scaling_mut(&mut self) -> &mut S {
        &mut self.scaling
    }

    pub fn set_scaling(&mut self, scaling: S) {
        self.scaling = scaling;
    }
}


impl<T: Zero, S: One> Transform2D<T, S> {
    pub fn identity() -> Self {
        Self {
            translation: Vec2::zero(),
            scaling: S::one(),
        }
    }
}

impl<T: Zero, S: One> Default for Transform2D<T, S> {
    fn default() -> Self {
        Self::identity()
    }
}