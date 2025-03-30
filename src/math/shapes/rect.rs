use std::{ops::Add, process::Output};

use crate::math::{Vec2, Transformable};

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

impl<T: Copy + Add<T, Output = T>> Rect<T> {
    pub fn x1(&self) -> T {
        *self.position.x()
    }

    pub fn x2(&self) -> T {
        *self.position.x() + *self.size.x()
    }

    pub fn y1(&self) -> T {
        *self.position.y()
    }

    pub fn y2(&self) -> T {
        *self.position.y() + *self.size.y()
    }
}

impl<T: Copy, S: Copy> Transformable<T, S> for Rect<T> where 
    Vec2<T>: Transformable<T, S>
{
    fn transform(&mut self, transform: &crate::math::Transform2D<T, S>) {
        self.position.transform(transform);
        self.size.scale(*transform.get_scaling());
    }

    fn translate(&mut self, offset: Vec2<T>) {
        self.position.translate(offset);
    }

    fn scale(&mut self, factor: S) {
        self.position.scale(factor);
        self.size.scale(factor);
    }
}
