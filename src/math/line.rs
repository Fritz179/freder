use super::{Transformable, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line<T = i32> {
    start: Vec2<T>,
    end: Vec2<T>,
}

impl<T> Line<T> {
    pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
        Self {
            start: Vec2::new(x1, y1),
            end: Vec2::new(x2, y2),
        }
    }
    
    pub fn new_vec(start: Vec2<T>, end: Vec2<T>) -> Self {
        Self { start, end }
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

impl<T, S> Transformable<T, S> for Line<T> where 
    Vec2<T>: Transformable<T, S>,
{
    fn translate(&mut self, offset: &Vec2<T>) {
        self.start.translate(offset);
        self.end.translate(offset);
    }

    fn scale(&mut self, factor: &Vec2<S>) {
        self.start.scale(factor);
        self.end.scale(factor);
    }
}
