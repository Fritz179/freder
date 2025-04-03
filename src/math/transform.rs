use std::ops::{AddAssign, MulAssign};

use super::{vector::Vector, One, Vec2, Zero};


pub trait Transformer<T = i32, const N: usize = 2> {    
    fn transform_vec(&self, vector: &mut Vector<T, N>);

    // Clone on the heap? Transformer is never sized...
    fn get_scaling_part(&self) -> Box<dyn Transformer<T, N>>;
    fn get_translating_part(&self) -> Box<dyn Transformer<T, N>>;

    fn is_axis_aligned(&self) -> bool;
}

pub trait Transform<T = i32, const N: usize = 2> {    
    fn transform(&mut self, vector: &dyn Transformer<T, N>);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transform2D<T = i32>{
    scaling: Vec2<T>,
    translation: Vec2<T>,
}

impl<T: Copy> Transform2D<T> {
    pub fn new(translation: Vec2<T>, scaling: Vec2<T>) -> Self {
        Self { translation, scaling }
    }

    pub fn translation(self) -> Vec2<T> {
        self.translation
    }

    pub fn translation_mut(&mut self) -> &mut Vec2<T> {
        &mut self.translation
    }

    pub fn scaling(&self) -> Vec2<T> {
        self.scaling
    }

    pub fn scaling_mut(&mut self) -> &mut Vec2<T> {
        &mut self.scaling
    }
}


impl<T: Zero + One> Transform2D<T> {
    pub fn identity() -> Self {
        Self {
            translation: Vec2::zero(),
            scaling: Vec2::one(),
        }
    }
}

impl<T: Zero + One> Default for Transform2D<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: Copy + One + Zero + 'static> Transformer<T> for Transform2D<T> where
    Vec2<T>: AddAssign<Vec2<T>>,
    Vec2<T>: MulAssign<Vec2<T>>
{
    fn transform_vec(&self, vector: &mut Vec2<T>) {
        *vector *= self.scaling;
        *vector += self.translation;
    }

    fn get_scaling_part(&self) -> Box<dyn Transformer<T, 2>> {
        Box::new(Self::new(Vec2::zero(), self.scaling))
    }

    fn get_translating_part(&self) -> Box<dyn Transformer<T, 2>> {
        Box::new(Self::new(self.translation, Vec2::one()))
    }

    fn is_axis_aligned(&self) -> bool {
        true
    }
}