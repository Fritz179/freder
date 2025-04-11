use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line<T = i32> {
    start: Vec2<T>,
    end: Vec2<T>,
}

impl<T: Number> Line<T> {
    pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
        Self {
            start: Vec2::new(x1, y1),
            end: Vec2::new(x2, y2),
        }
    }
    
    pub fn new_vec(start: Vec2<T>, end: Vec2<T>) -> Self {
        Self { start, end }
    }

    pub fn new_slice(slice: [Vec2<T>; 2]) -> Self {
        Self {
            start: slice[0],
            end: slice[1],
        }
    }

    pub fn start(&self) -> &Vec2<T> {
        &self.start
    }

    pub fn start_mut(&mut self) -> &mut Vec2<T> {
        &mut self.start
    }

    pub fn set_start(&mut self, start: Vec2<T>) {
        self.start = start;
    }

    pub fn end(&self) -> &Vec2<T> {
        &self.end
    }

    pub fn end_mut(&mut self) -> &mut Vec2<T> {
        &mut self.end
    }

    pub fn set_end(&mut self, end: Vec2<T>) {
        self.end = end;
    }

    pub fn as_tuple(&self) -> ((&T, &T), (&T, &T)) {
        (self.start.as_tuple(), self.end.as_tuple())
    }

    pub fn to_tuple(self) -> ((T, T), (T, T)) {
        (self.start.to_tuple(), self.end.to_tuple())
    }
}

impl<T: Copy> Transform<T, 2> for Line<T> where 
    Vec2<T>: Transform<T, 2>,
{
    fn transform(&mut self, transform: &dyn Transformer<T, 2>) {
        self.start.transform(transform);
        self.end.transform(transform);
    }
}
