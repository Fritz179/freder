use std::ops::{AddAssign, MulAssign};

use super::{vector::Vector, One, Vec2, Zero};


pub trait Transformer<T = i32, const N: usize = 2> {    
    fn transform_vec(&self, vector: &mut Vector<T, N>) where
    Vec2<T>: AddAssign<Vec2<T>>,
    Vec2<T>: MulAssign<Vec2<T>>;

    // Clone on the heap? Transformer is never sized...
    fn scaling(&self) -> Vec2<T>;
    fn translation(&self) -> Vec2<T>;

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

impl<T: Copy + Zero + One> Transform2D<T> {
    pub fn new(translation: Vec2<T>, scaling: Vec2<T>) -> Self {
        Self { translation, scaling }
    }

    pub fn new_scaling(scaling: Vec2<T>) -> Self {
        Self { translation: Default::default(), scaling }
    }

    pub fn new_translation(translation: Vec2<T>) -> Self {
        Self { translation, scaling: Default::default() }
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

    pub fn identity() -> Self {
        Self {
            translation: Vec2::zero(),
            scaling: Vec2::one(),
        }
    }
}

impl<T: Copy + Zero + One> Default for Transform2D<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: Copy + One + Zero + 'static> Transformer<T> for Transform2D<T> {
    fn transform_vec(&self, vector: &mut Vec2<T>) where
        Vec2<T>: AddAssign<Vec2<T>>,
        Vec2<T>: MulAssign<Vec2<T>> 
    {
        *vector *= self.scaling;
        *vector += self.translation;
    }

    fn scaling(&self) -> Vec2<T> {
        self.scaling
    }

    fn translation(&self) -> Vec2<T> {
        self.translation
    }

    fn is_axis_aligned(&self) -> bool {
        true
    }
}