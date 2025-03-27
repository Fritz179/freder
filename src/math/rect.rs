use super::{Transformable, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect<T = i32> {
    position: Vec2<T>,
    size: Vec2<T>,
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }

    pub fn new_vec(position: Vec2<T>, size: Vec2<T>) -> Self {
        Self { position, size }
    }
}

impl<T: Copy, S: Copy> Transformable<T, S> for Rect<T> where 
    Vec2<T>: Transformable<T, S>,
{
    fn translate(&mut self, offset: Vec2<T>) {
        self.position.translate(offset);
    }

    fn scale(&mut self, factor: S) {
        self.position.scale(factor);
        self.size.scale(factor);
    }
}
