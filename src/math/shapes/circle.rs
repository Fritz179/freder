use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Circle<T: Number = i32> {
    center: Vec2<T>,
    radius: T,
}

impl<T: Number> Circle<T> {
    pub fn new(x: T, y: T, radius: T) -> Self {
        Self { center: Vec2::new(x, y), radius }
    }

    pub fn center(&self) -> &Vec2<T> {
        &self.center
    }

    pub fn radius(&self) -> T {
        self.radius
    }
}

impl<T: Number> Transform<T, 2> for Circle<T> {
    fn transform(&mut self, transform: &dyn Transformer<T, 2>) {
        self.center.transform(transform);
        self.radius *= *transform.scaling().x();
    }
}