use crate::prelude::*;

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

impl<T: Copy> Transform<T, 2> for Triangle<T> where 
    Vec2<T>: Transform<T, 2>,
{
    fn transform(&mut self, transform: &dyn Transformer<T, 2>) {
        self.a.transform(transform);
        self.b.transform(transform);
        self.c.transform(transform);
    }
}
