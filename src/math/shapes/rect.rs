use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect<T = i32> {
    position: Vec2<T>,
    size: Vec2<T>,
}

impl<T: Number> Rect<T> {
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

impl<T: Number> Rect<T> {
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

impl<T: Number> Transform<T, 2> for Rect<T> {
    fn transform(&mut self, transform: &dyn Transformer<T, 2>) {
        self.position.transform(transform);
        self.size.transform(&Transform2D::new_scaling(transform.scaling()));
    }
}
