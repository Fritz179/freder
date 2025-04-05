use crate::prelude::*;
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T: Zero, const N: usize> Zero for Vector<T, N> {
    const ZERO: Self = Self ([T::ZERO; N]);

    fn zero() -> Self {
        Self ([T::ZERO; N])
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|x| x.is_zero())
    }
}

impl<T: Zero, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: One, const N: usize> One for Vector<T, N> {
    const ONE: Self = Self ([T::ONE; N]);

    fn one() -> Self {
        Self ([T::ONE; N])
    }

    fn is_one(&self) -> bool {
        self.0.iter().all(|x| x.is_one())
    }
}

pub type Vec2<T = i32> = Vector<T, 2>;

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self ([x, y])
    }

    pub fn x(&self) -> &T {
        &self.0[0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    pub fn set_x(&mut self, x: T) {
        self.0[0] = x;
    }

    pub fn y(&self) -> &T {
        &self.0[1]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    pub fn set_y(&mut self, y: T) {
        self.0[1] = y;
    }

    pub fn as_tuple(&self) -> (&T, &T) {
        (&self.0[0], &self.0[1])
    }

    pub fn to_tuple(self) -> (T, T) {
        self.0.into()
    }
}

macro_rules! vector_binary_op_impl {
    ($trait:ident, $func:ident, $op:tt, $trait_assign:ident, $func_assign:ident, $op_assign:tt) => {

        // a + b
        impl<Lhs, Rhs, const N: usize> std::ops::$trait<Vector<Rhs, N>> for Vector<Lhs, N>
        where
            Lhs: std::ops::$trait<Rhs>,
        {
            type Output = Vector<<Lhs as std::ops::$trait<Rhs>>::Output, N>;

            fn $func(self, rhs: Vector<Rhs, N>) -> Self::Output {
                let mut temp: [MaybeUninit<<Lhs as std::ops::$trait<Rhs>>::Output>; N] = [const { MaybeUninit::uninit() }; N];

                for (t, (a, b)) in temp.iter_mut().zip(self.0.into_iter().zip(rhs.0.into_iter())) {
                    t.write(a $op b);
                }

                // let arr = unsafe { MaybeUninit::array_assume_init(temp) };

                // SAFETY: The array is initialized and has the correct legth N
                let arr = unsafe { std::mem::transmute_copy(&temp) };
                Vector(arr)
            }
        }

        // a + &b
        impl<'a, Lhs, Rhs, const N: usize> std::ops::$trait<&'a Vector<Rhs, N>> for Vector<Lhs, N>
        where
            Lhs: std::ops::$trait<&'a Rhs>,
        {
            type Output = Vector<<Lhs as std::ops::$trait<&'a Rhs>>::Output, N>;

            fn $func(self, rhs: &'a Vector<Rhs, N>) -> Self::Output {
                let mut temp: [MaybeUninit<<Lhs as std::ops::$trait<&'a Rhs>>::Output>; N] = [const { MaybeUninit::uninit() }; N];

                for (t, (a, b)) in temp.iter_mut().zip(self.0.into_iter().zip(rhs.0.iter())) {
                    t.write(a $op b);
                }

                // SAFETY: The array is initialized and has the correct legth N
                let arr = unsafe { std::mem::transmute_copy(&temp) };
                Vector(arr)
            }
        }

        // &a + &b
        impl<'a, Lhs, Rhs, const N: usize> std::ops::$trait<&'a Vector<Rhs, N>> for &'a Vector<Lhs, N>
        where
            &'a Lhs: std::ops::$trait<&'a Rhs>,
        {
            type Output = Vector<<&'a Lhs as std::ops::$trait<&'a Rhs>>::Output, N>;

            fn $func(self, rhs: &'a Vector<Rhs, N>) -> Self::Output {
                let mut temp: [MaybeUninit<<&'a Lhs as std::ops::$trait<&'a Rhs>>::Output>; N] = [const { MaybeUninit::uninit() }; N];

                for (t, (a, b)) in temp.iter_mut().zip(self.0.iter().zip(rhs.0.iter())) {
                    t.write(a $op b);
                }

                // SAFETY: The array is initialized and has the correct legth N
                let arr = unsafe { std::mem::transmute_copy(&temp) };
                Vector(arr)
            }
        }

        // a += b
        impl<Lhs, Rhs> std::ops::$trait_assign<Vec2<Rhs>> for Vec2<Lhs>
        where
            Lhs: std::ops::$trait_assign<Rhs>,
        {
            fn $func_assign(&mut self, rhs: Vec2<Rhs>) {
                for (a, b) in self.0.iter_mut().zip(rhs.0.into_iter()) {
                    *a $op_assign b;
                }
            }
        }

        // a += &b
        impl<Lhs, Rhs> std::ops::$trait_assign<&Vec2<Rhs>> for Vec2<Lhs>
        where
            Lhs: for<'a> std::ops::$trait_assign<&'a Rhs>,
        {
            fn $func_assign(&mut self, rhs: &Vec2<Rhs>) {
                for (a, b) in self.0.iter_mut().zip(rhs.0.iter()) {
                    *a $op_assign b;
                }
            }
        }
    };
}

vector_binary_op_impl!(Add, add, +, AddAssign, add_assign, +=);
vector_binary_op_impl!(Sub, sub, -, SubAssign, sub_assign, -=);
vector_binary_op_impl!(Mul, mul, *, MulAssign, mul_assign, *=);

// TODO: Implement more operations

// macro_rules! vector_float_mul_impl {
//     ($Lhs:ty, $Rhs:ty, $to:tt, $from:tt) => {
//         impl<const N: usize> Mul<$Rhs> for Vector<$Lhs, N> {
//             type Output = Self;

//             fn mul(self, factor: $Rhs) -> Self::Output {
//                 let mut temp: [MaybeUninit<$Lhs>; N] = [const { MaybeUninit::uninit() }; N];

//                 for (t, a) in temp.iter_mut().zip(self.0.into_iter()) {
//                     t.write(<$Lhs>::$from(a.$to().expect("Conversion failed") * factor).expect("Conversion failed"));
//                 }

//                 // let arr = unsafe { MaybeUninit::array_assume_init(temp) };
//                 let arr = unsafe { std::mem::transmute_copy(&temp) };
//                 Vector(arr)
//             }
//         }

//         // Vec * f
//         impl MulAssign<$Rhs> for Vec2<$Lhs> {
//             fn mul_assign(&mut self, factor: $Rhs) {
//                 // self.0[0] *_assign $Rhs.x;
//                 // self.0[1] *_assign $Rhs.y;
//                 for a in self.0.iter_mut() {
//                     *a = <$Lhs>::$from(a.$to().expect("Conversion failed") * factor).expect("Conversion failed");
//                 }
//             }
//         }

//         // Vec * &f
//         impl MulAssign<&$Rhs> for Vec2<$Lhs> {
//             fn mul_assign(&mut self, factor: &$Rhs) {
//                 for a in self.0.iter_mut() {
//                     *a = <$Lhs>::$from(a.$to().expect("Conversion failed") * factor).expect("Conversion failed");
//                 }
//             }
//         }
//     };
// }

// vector_float_mul_impl!(i32, f32, to_f32, from_f32);
// vector_float_mul_impl!(i64, f64, to_f64, from_f64);

// use num_traits::{FromPrimitive, ToPrimitive};

impl<T: Number, const N: usize> Transform<T, N> for Vector<T, N> {
    fn transform(&mut self, transform: &dyn Transformer<T, N>) {
        transform.transform_vec(self);
    }
}

// impl<V: Copy, S: Copy> std::ops::Mul<Transform2D<V, S>> for Vec2<V> where 
//     Vec2<V>: Transformable<V, S>,
// {
//     type Output = Vec2<V>;

//     fn mul(self, rhs: Transform2D<V, S>) -> Self::Output {
//         let mut vec = self;
//         vec.transform(&rhs);
//         vec
//     }
// }

// impl<V: Copy, S: Copy> std::ops::MulAssign<Transform2D<V, S>> for Vec2<V> where 
//     Vec2<V>: Transformable<V, S>,
// {
//     fn mul_assign(&mut self, rhs: Transform2D<V, S>) {
//         self.transform(&rhs);
//     }
// }