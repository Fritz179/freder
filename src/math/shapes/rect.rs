use std::ops::Add;

use crate::math::{Transform, Vec2};

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

impl<T: Copy> Transform<T, 2> for Rect<T> where 
    Vec2<T>: Transform<T, 2>
{
    fn transform(&mut self, transform: &dyn crate::math::Transformer<T, 2>) {
        self.position.transform(transform);
        self.size.transform(transform.get_scaling_part().as_ref());
    }
}
