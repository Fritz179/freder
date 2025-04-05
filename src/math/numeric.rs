// Number:
//   - Float 
//     - f32
//     - f64
//   - Integer
//     - Signed
//       - i8...
//     - Unsigned
//       - u8...

use std::ops::{Add, AddAssign, Mul, MulAssign};


/// Unsigned integer types
pub trait Unsigned: Integer {}

macro_rules! do_unsigned_impl {
    ($m:tt $(, $val:expr)*) => {
        $m!(usize $(, $val)*);
        $m!(u8 $(, $val)*);
        $m!(u16 $(, $val)*);
        $m!(u32 $(, $val)*);
        $m!(u64 $(, $val)*);
        $m!(u128 $(, $val)*);
    };
}

macro_rules! unsigned_impl {
    ($ty:tt) => {
        impl Unsigned for $ty {}
    }
}

do_unsigned_impl!(unsigned_impl);


/// Signed integer types
pub trait Signed: Integer {}

macro_rules! do_signed_impl {
    ($m:tt $(, $val:expr)*) => {
        $m!(isize $(, $val)*);
        $m!(i8 $(, $val)*);
        $m!(i16 $(, $val)*);
        $m!(i32 $(, $val)*);
        $m!(i64 $(, $val)*);
        $m!(i128 $(, $val)*);
    };
}

macro_rules! signed_impl {
    ($ty:tt) => {
        impl Signed for $ty {}
    }
}

do_signed_impl!(signed_impl);


/// Integer types
pub trait Integer: Number {}

macro_rules! do_integer_impl {
    ($m:tt $(, $val:expr)*) => {
        do_signed_impl!($m $(, $val)*);
        do_unsigned_impl!($m $(, $val)*);
    };
}

macro_rules! integer_impl {
    ($t:ty $(, $val:expr)*) => {
        impl Integer for $t {}
    };
}

do_integer_impl!(integer_impl);


/// Floating point types
pub trait Float: Number {}

macro_rules! do_float_impl {
    ($m:tt $(, $val:expr)*) => {
        $m!(f32 $(, $val)*);
        $m!(f64 $(, $val)*);
    };
}

macro_rules! float_impl {
    ($t:ty) => {
        impl Float for $t {}
    };
}

do_float_impl!(float_impl);


/// Number trait
pub trait Number: 
    Sized +
    Add<Output = Self> +
    AddAssign +
    Mul<Output = Self> +
    MulAssign +
    Zero +
    One +
    Copy {}

macro_rules! do_number_impl {
    ($m:tt $(, $val:expr)*) => {
        do_integer_impl!($m $(, $val)*);
        do_float_impl!($m $(, $val)*);
    };
}

macro_rules! number_impl {
    ($t:ty) => {
        impl Number for $t {}
    };
}

do_number_impl!(number_impl);


/// Zero
pub trait Zero {
    const ZERO: Self;

    fn zero() -> Self;

    fn is_zero(&self) -> bool;

    fn set_zero(&mut self) where Self: Sized {
        *self = Self::zero();
    }   
}

macro_rules! zero_impl { 
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            const ZERO: $t = $v;

            #[inline]
            fn zero() -> $t {
                $v
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == $v
            }
        }
    };
}

do_integer_impl!(zero_impl, 0);
do_float_impl!(zero_impl, 0.0);

pub trait One {
    const ONE: Self;

    fn one() -> Self;

    fn is_one(&self) -> bool;

    fn set_one(&mut self) where Self: Sized {
        *self = Self::one();
    }   
}

macro_rules! one_impl { 
    ($t:ty, $v:expr) => {
        impl One for $t {
            const ONE: $t = $v;

            #[inline]
            fn one() -> $t {
                $v
            }

            #[inline]
            fn is_one(&self) -> bool {
                *self == $v
            }
        }
    };
}

do_integer_impl!(one_impl, 1);
do_float_impl!(one_impl, 1.0);