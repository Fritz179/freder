use crate::math::{Transform2D, Transformable, Vec2};

pub struct Triangle<T = i32> {
    a: Vec2<T>,
    b: Vec2<T>,
    c: Vec2<T>,
}

impl<T> Triangle<T> {
    pub fn new(a: Vec2<T>, b: Vec2<T>, c: Vec2<T>) -> Self {
        Self { a, b, c }
    }
}

impl<T: Copy, S: Copy> Transformable<T, S> for Triangle<T> where 
    Vec2<T>: Transformable<T, S>,
{
    fn transform(&mut self, transform: &Transform2D<T, S>) {
        self.a.transform(transform);
        self.b.transform(transform);
        self.c.transform(transform);
    }

    fn translate(&mut self, offset: Vec2<T>) {
        self.a.translate(offset);
        self.b.translate(offset);
        self.c.translate(offset);
    }

    fn scale(&mut self, factor: S) {
        self.a.scale(factor);
        self.b.scale(factor);
        self.c.scale(factor);
    }
}
