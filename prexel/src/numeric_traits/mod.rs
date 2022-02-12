
/// Implements unchecked numeric traits (Add, Sub, Mul, Div, Rem, Neg) for `T` using one of its
/// field where `T` implements `From` taking a the field type: `T::From(typeof field)`.
#[macro_export]
macro_rules! impl_unchecked_num_traits_with_field {
    ($type:ty => $field:tt, $field_type:ty) => {
        impl std::ops::Add for $type {
            type Output = Self;

            #[inline]
            fn add(self, other: Self) -> Self {
                let x = self.$field;
                let y = other.$field;
                Self::from(x + y)
            }
        }

        impl std::ops::Sub for $type {
            type Output = Self;

            #[inline]
            fn sub(self, other: Self) -> Self {
                let x = self.$field;
                let y = other.$field;
                Self::from(x + y)
            }
        }

        impl std::ops::Mul for $type {
            type Output = Self;

            #[inline]
            fn mul(self, other: Self) -> Self {
                let x = self.$field;
                let y = other.$field;
                Self::from(x * y)
            }
        }

        impl std::ops::Div for $type {
            type Output = Self;

            #[inline]
            fn div(self, other: Self) -> Self {
                let x = self.$field;
                let y = other.$field;
                Self::from(x / y)
            }
        }

        impl std::ops::Rem for $type {
            type Output = Self;

            #[inline]
            fn rem(self, other: Self) -> Self {
                let x = self.$field;
                let y = other.$field;
                Self::from(x % y)
            }
        }

        impl std::ops::Neg for $type {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                let x = self.$field;
                Self::from(-x)
            }
        }

        impl $crate::num_traits::Zero for $type {
            #[inline]
            fn zero() -> Self {
                Self::from(0)
            }

            #[inline]
            fn is_zero(&self) -> bool {
                self.$field.is_zero()
            }
        }

        impl $crate::num_traits::One for $type {
            #[inline]
            fn one() -> Self {
                Self::from(1)
            }
        }

        impl std::cmp::PartialOrd for $type {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                let x = self.$field;
                let y = other.$field;
                x.partial_cmp(&y)
            }
        }

        impl std::cmp::PartialEq for $type {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                let x = self.$field;
                let y = other.$field;
                x == y
            }
        }

        impl $crate::num_traits::ToPrimitive for $type {
            #[inline]
            fn to_i64(&self) -> Option<i64> {
                self.$field.to_i64()
            }

            #[inline]
            fn to_u64(&self) -> Option<u64> {
                self.$field.to_u64()
            }

            #[inline]
            fn to_f64(&self) -> Option<f64> {
                self.$field.to_f64()
            }
        }

        impl $crate::num_traits::FromPrimitive for $type {
            #[inline]
            fn from_i64(n: i64) -> Option<Self> {
                <$field_type>::from_i64(n).map(|x| Self::from(x))
            }

            #[inline]
            fn from_u64(n: u64) -> Option<Self> {
                <$field_type>::from_u64(n).map(|x| Self::from(x))
            }

            #[inline]
            fn from_f64(n: f64) -> Option<Self> {
                <$field_type>::from_f64(n).map(|x| Self::from(x))
            }
        }

        impl std::clone::Clone for $type {
            #[inline]
            fn clone(&self) -> Self {
                Self::from(self.$field)
            }
        }

        impl std::fmt::Display for $type {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.$field.fmt(f)
            }
        }

        impl std::fmt::Debug for $type {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.$field.fmt(f)
            }
        }

        #[cfg(debug_assertions)]
        const _ : () = {
          fn assert_implements_from_type_of_type(x: $type) {
              struct AssertImplFrom<T: From<U>, U>(T, std::marker::PhantomData<U>);

              let _: $field_type = x.$field;
              let _: AssertImplFrom<$type, $field_type> = AssertImplFrom(x, std::marker::PhantomData);
          }
        };
    }
}

/// Implements checked numeric traits (CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, CheckedRem, CheckedNeg)
/// for `T` using one of its field where `T` implements `From` taking a the field type: `T::From(typeof field)`.
#[macro_export]
macro_rules! impl_checked_num_traits_with_field {
    ($type:ty => $field:tt, $field_type:ty) => {
        $crate::impl_unchecked_num_traits_with_field!($type => $field, $field_type);

        impl $crate::num::checked::CheckedAdd for $type {
            #[inline]
            fn checked_add(&self, other: &Self) -> Option<Self> {
                let x = self.$field;
                let y = other.$field;
                $crate::num::checked::CheckedAdd::checked_add(&x, &y).map(Self::from)
            }
        }

        impl $crate::num::checked::CheckedSub for $type {
            #[inline]
            fn checked_sub(&self, other: &Self) -> Option<Self> {
                let x = self.$field;
                let y = other.$field;
                $crate::num::checked::CheckedSub::checked_sub(&x, &y).map(Self::from)
            }
        }

        impl $crate::num::checked::CheckedMul for $type {
            #[inline]
            fn checked_mul(&self, other: &Self) -> Option<Self> {
                let x = self.$field;
                let y = other.$field;
                $crate::num::checked::CheckedMul::checked_mul(&x, &y).map(Self::from)
            }
        }

        impl $crate::num::checked::CheckedDiv for $type {
            #[inline]
            fn checked_div(&self, other: &Self) -> Option<Self> {
                let x = self.$field;
                let y = other.$field;
                $crate::num::checked::CheckedDiv::checked_div(&x, &y).map(Self::from)
            }
        }

        impl $crate::num::checked::CheckedRem for $type {
            #[inline]
            fn checked_rem(&self, other: &Self) -> Option<Self> {
                let x = self.$field;
                let y = other.$field;
                $crate::num::checked::CheckedRem::checked_rem(&x, &y).map(Self::from)
            }
        }

        impl $crate::num::checked::CheckedNeg for $type {
            #[inline]
            fn checked_neg(&self) -> Option<Self> {
                let x = self.$field;
                $crate::num::checked::CheckedNeg::checked_neg(&x).map(Self::from)
            }
        }
    }
}

/// Implements unchecked numeric traits (Add, Sub, Mul, Div, Rem, Neg) and `FromStr` for `T` using one of its
/// field where `T` implements `From` taking a the field type: `T::From(typeof field)`.
#[macro_export]
macro_rules! impl_unchecked_num_traits_with_field_and_from_str {
    ($type:ty => $field:tt, $field_type:ty) => {
        $crate::impl_unchecked_num_traits_with_field!($type => $field, $field_type);

        impl std::str::FromStr for $type {
            type Err = <$field_type as std::str::FromStr>::Err;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let x : $field_type = std::str::FromStr::from_str(s)?;
                Ok(Self::from(x))
            }
      }
    }
}

/// Implements checked numeric traits (CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, CheckedRem, CheckedNeg)
/// and `FromStr` for `T` using one of its field where `T` implements `From` taking a the field type: `T::From(typeof field)`.
#[macro_export]
macro_rules! impl_checked_num_traits_with_field_and_from_str {
    ($type:ty => $field:tt, $field_type:ty) => {
        $crate::impl_checked_num_traits_with_field!($type => $field, $field_type);

        impl std::str::FromStr for $type {
            type Err = <$field_type as std::str::FromStr>::Err;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let x : $field_type = std::str::FromStr::from_str(s)?;
                Ok(Self::from(x))
            }
      }
    }
}