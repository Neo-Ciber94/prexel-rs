
/// Provides traits for implements a checked numeric types.
pub mod checked {
    use num_traits::{FromPrimitive, ToPrimitive, Zero, One};
    use std::fmt::{Debug, Display};
    use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
    use std::str::FromStr;

    /// A base trait for numeric types, provides `0` and `1` values,
    /// comparisons, conversions from and to string, and checked numeric operations.
    ///
    /// This trait is implemented for all the types that implements the traits.
    pub trait CheckedNum: CheckedNumOps
    + Zero
    + One
    + PartialOrd
    + ToPrimitive
    + FromPrimitive
    + FromStr
    + Clone
    + Debug
    + Display
    {
    }

    impl<T: Sized> CheckedNum for T where
        T: CheckedNumOps
        + Zero
        + One
        + PartialOrd
        + FromPrimitive
        + ToPrimitive
        + FromStr
        + Clone
        + Debug
        + Display
    {
    }

    /// Traits for basic checked numeric operations that returns `None` on overflow.
    ///
    /// This trait is implemented for all the types that implements the traits.
    pub trait CheckedNumOps:
        CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem + CheckedNeg
    {
    }

    impl<T: Sized> CheckedNumOps for T where
        T: CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem + CheckedNeg
    {
    }

    /// Performs an addiction that returns `None` if the result overflow/underflow.
    pub trait CheckedAdd: Sized + Add<Output = Self> {
        /// Adds two values and returns `None` if the result overflow/underflow.
        fn checked_add(&self, other: &Self) -> Option<Self>;
    }

    /// Performs a subtraction that returns `None` if the result overflow/underflow.
    pub trait CheckedSub: Sized + Sub<Output = Self> {
        /// Subtracts two values and returns `None` if the result overflow/underflow.
        fn checked_sub(&self, other: &Self) -> Option<Self>;
    }

    /// Performs a multiplication that returns `None` if the result overflow/underflow.
    pub trait CheckedMul: Sized + Mul<Output = Self> {
        /// Multiplies two values and returns `None` if the result overflow/underflow.
        fn checked_mul(&self, other: &Self) -> Option<Self>;
    }

    /// Performs a division that returns `None` if the divisor is zero or the result overflow/underflow.
    pub trait CheckedDiv: Sized + Div<Output = Self> {
        /// Divides two values and returns `None` if the divisor is zero or the result overflow/underflow.
        fn checked_div(&self, other: &Self) -> Option<Self>;
    }

    /// Performs an integral remainder that returns `None` if the divisor is zero or the result overflow/underflow.
    pub trait CheckedRem: Sized + Rem<Output = Self> {
        /// Gets the remainder two values and returns `None` if the divisor is zero or the result overflow/underflow.
        fn checked_rem(&self, other: &Self) -> Option<Self>;
    }

    /// Performs a negation that returns `None` if the result overflow/underflow.
    pub trait CheckedNeg: Sized + Neg<Output = Self> {
        /// Negates a value and returns`None` if the divisor is zero or the result overflow/underflow.
        fn checked_neg(&self) -> Option<Self>;
    }

    //////////////////////// Macros ////////////////////////

    #[macro_export]
    macro_rules! unsafe_impl_checked_ops {
    ($($type:ty), *) => ($(
        impl CheckedAdd for $type{
            #[inline]
            fn checked_add(&self, other: &Self) -> Option<Self>{
                Some(self + other)
            }
        }

        impl CheckedSub for $type{
            #[inline]
            fn checked_sub(&self, other: &Self) -> Option<Self>{
                Some(self - other)
            }
        }

        impl CheckedMul for $type{
            #[inline]
            fn checked_mul(&self, other: &Self) -> Option<Self>{
                Some(self * other)
            }
        }

        impl CheckedDiv for $type{
            #[inline]
            fn checked_div(&self, other: &Self) -> Option<Self>{
                if num_traits::Zero::is_zero(other){
                    return None;
                }
                Some(self / other)
            }
        }

        impl CheckedRem for $type{
            #[inline]
            fn checked_rem(&self, other: &Self) -> Option<Self>{
                if num_traits::Zero::is_zero(other){
                    return None;
                }
                Some(self % other)
            }
        }

        impl CheckedNeg for $type{
            #[inline]
            fn checked_neg(&self) -> Option<Self>{
                Some(-*self)
            }
        }
    )*)
}

    #[macro_export]
    macro_rules! impl_checked_ops {
    ($($type:ty), *) => ($(
        impl CheckedAdd for $type{
            #[inline]
            fn checked_add(&self, other: &Self) -> Option<Self>{
                <$type>::checked_add(*self, *other)
            }
        }

        impl CheckedSub for $type{
            #[inline]
            fn checked_sub(&self, other: &Self) -> Option<Self>{
                <$type>::checked_sub(*self, *other)
            }
        }

        impl CheckedMul for $type{
            #[inline]
            fn checked_mul(&self, other: &Self) -> Option<Self>{
                <$type>::checked_mul(*self, *other)
            }
        }

        impl CheckedDiv for $type{
            #[inline]
            fn checked_div(&self, other: &Self) -> Option<Self>{
                <$type>::checked_div(*self, *other)
            }
        }

        impl CheckedRem for $type{
            #[inline]
            fn checked_rem(&self, other: &Self) -> Option<Self>{
                <$type>::checked_rem(*self, *other)
            }
        }

        impl CheckedNeg for $type{
            #[inline]
            fn checked_neg(&self) -> Option<Self>{
                <$type>::checked_neg(*self)
            }
        }
    )*)
}

    #[macro_export]
    macro_rules! impl_checked_binary {
    ($($type:ty), *) => ($(
        impl CheckedAdd for $type{
            #[inline]
            fn checked_add(&self, other: &Self) -> Option<Self>{
                <$type>::checked_add(*self, *other)
            }
        }

        impl CheckedSub for $type{
            #[inline]
            fn checked_sub(&self, other: &Self) -> Option<Self>{
                <$type>::checked_sub(*self, *other)
            }
        }

        impl CheckedMul for $type{
            #[inline]
            fn checked_mul(&self, other: &Self) -> Option<Self>{
                <$type>::checked_mul(*self, *other)
            }
        }

        impl CheckedDiv for $type{
            #[inline]
            fn checked_div(&self, other: &Self) -> Option<Self>{
                <$type>::checked_div(*self, *other)
            }
        }

        impl CheckedRem for $type{
            #[inline]
            fn checked_rem(&self, other: &Self) -> Option<Self>{
                <$type>::checked_rem(*self, *other)
            }
        }
    )*)
}

    #[macro_export]
    macro_rules! impl_checked_unary{
    ($($type:ty), *) => ($(
        impl CheckedNeg for $type{
            #[inline]
            fn checked_neg(&self) -> Option<Self>{
                <$type>::checked_neg(*self)
            }
        }
    )*)
}

    // Implementing all the Checked operations
    impl_checked_ops!(i8, i16, i32, i64, isize);

    // Implementing only all the Checked binary operations
    impl_checked_binary!(u8, u16, u32, u64, usize);

    // Implementing all the Checked operations by forwarding to the corresponding std::ops,
    // necessary to have compatibility with f32 y f64.
    unsafe_impl_checked_ops!(f32, f64);
}

/// Provides traits for implements a unchecked numeric types.
pub mod unchecked {
    use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
    use std::fmt::{Debug, Display};
    use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
    use std::str::FromStr;

    /// Traits for basic unchecked numeric operations.
    ///
    /// This trait is implemented for all the types that implements the traits.
    pub trait UncheckedNum:
        UncheckedNumOps
        + Zero
        + One
        + PartialOrd
        + FromStr
        + ToPrimitive
        + FromPrimitive
        + Clone
        + Display
        + Debug
    {
    }

    impl<T> UncheckedNum for T where
        T: UncheckedNumOps
            + Zero
            + One
            + PartialOrd
            + FromStr
            + ToPrimitive
            + FromPrimitive
            + Clone
            + Display
            + Debug
    {
    }

    /// Traits for basic unchecked numeric operations.
    ///
    /// This trait is implemented for all the types that implements the traits.
    pub trait UncheckedNumOps:
        Sized
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Rem<Self, Output = Self>
        + Neg<Output = Self>
    {
    }

    impl<T> UncheckedNumOps for T where
        T: Add<Self, Output = Self>
            + Sub<Self, Output = Self>
            + Mul<Self, Output = Self>
            + Div<Self, Output = Self>
            + Rem<Self, Output = Self>
            + Neg<Output = Self>
    {
    }
}
