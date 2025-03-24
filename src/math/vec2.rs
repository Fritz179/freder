use std::fmt::Debug;

use super::{One, Transform, Transformable, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T = i32> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    pub fn as_tuple(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Zero> Zero for Vec2<T> {
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero()
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: One> One for Vec2<T> {
    fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one()
        }
    }

    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_one()
    }
}

impl<T: Zero> Default for Vec2<T> {
    fn default() -> Self {
        Self::zero()
    }
}

macro_rules! vec2_binary_op_impl {
    ($trait:ident, $func:ident, $op:tt, $trait_assign:ident, $func_assign:ident, $op_assign:tt) => {
        impl<Lhs, Rhs> std::ops::$trait<Vec2<Rhs>> for Vec2<Lhs>
        where
            Lhs: std::ops::$trait<Rhs>,
        {
            type Output = Vec2<<Lhs as std::ops::$trait<Rhs>>::Output>;

            fn $func(self, rhs: Vec2<Rhs>) -> Self::Output {
                Vec2 {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                }
            }
        }

        impl<Lhs, Rhs> std::ops::$trait_assign<Vec2<Rhs>> for Vec2<Lhs>
        where
            Lhs: std::ops::$trait_assign<Rhs>,
        {
            fn $func_assign(&mut self, rhs: Vec2<Rhs>) {
                self.x $op_assign rhs.x;
                self.y $op_assign rhs.y;
            }
        }
    };
}

vec2_binary_op_impl!(Add, add, +, AddAssign, add_assign, +=);
vec2_binary_op_impl!(Sub, sub, -, SubAssign, sub_assign, -=);
vec2_binary_op_impl!(Mul, mul, *, MulAssign, mul_assign, *=);

// TODO: Implement more operations


use num_traits::{FromPrimitive, ToPrimitive};

impl<V, T, S> Transformable<T, S> for Vec2<V>
where
    V: std::ops::AddAssign<T>,
    V: ToPrimitive + FromPrimitive,
    S: ToPrimitive,
{
    fn translate(&mut self, offset: Vec2<T>) {
        *self += offset;
    }

    fn scale(&mut self, factor: Vec2<S>) {
        let x_val = self.x.to_f64().expect("Conversion failed");
        let factor_x = factor.x.to_f64().expect("Conversion failed");
        let new_x = (x_val * factor_x).round();
        self.x = V::from_f64(new_x).expect("Conversion failed");

        let y_val = self.y.to_f64().expect("Conversion failed");
        let factor_y = factor.y.to_f64().expect("Conversion failed");
        let new_y = (y_val * factor_y).round();
        self.y = V::from_f64(new_y).expect("Conversion failed");
    }
}

impl<V, S> std::ops::Mul<Transform<V, S>> for Vec2<V> where 
    Vec2<V>: Transformable<V, S>,
{
    type Output = Vec2<V>;

    fn mul(self, rhs: Transform<V, S>) -> Self::Output {
        let mut vec = self;
        vec.transform(rhs);
        vec
    }
}

impl<V, S> std::ops::MulAssign<Transform<V, S>> for Vec2<V> where 
    Vec2<V>: Transformable<V, S>,
{
    fn mul_assign(&mut self, rhs: Transform<V, S>) {
        self.transform(rhs);
    }
}