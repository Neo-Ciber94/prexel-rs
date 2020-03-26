use std::marker::PhantomData;
use std::fmt::{Pointer, Formatter, Debug};

/// Represents a pointer to an arbitrary type, equivalent to a C `void*`.
///
/// # Examples
/// ```
/// use math_engine::utils::untyped::Untyped;
///
/// let p = Untyped::new(&10);
/// assert_eq!(10, unsafe { *p.cast::<i32>() });
/// ```
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Untyped<'a>{
    /// The raw pointer.
    pointer: *const (),
    _marker: &'a PhantomData<()>
}

impl <'a> Untyped<'a>{
    /// Constructs a new pointer to the given reference.
    #[inline]
    pub const fn new<T>(value: &'a T) -> Untyped<'a>{
        let raw = value as *const _ as *const ();
        Untyped {
            pointer: raw,
            _marker: &PhantomData
        }
    }

    /// Constructs a pointer using the given.
    #[inline]
    pub const unsafe fn new_unchecked<T>(ptr: *mut T) -> Untyped<'a>{
        Untyped {
            pointer: ptr as _,
            _marker: &PhantomData
        }
    }

    /// Converts this untyped to a raw pointer.
    #[inline]
    pub const fn into_raw(self) -> *const (){
        self.pointer
    }

    /// Cast this pointer to the given type and get a reference to it.
    ///
    /// # Remarks
    /// If the target type size is greater than the pointed value the result is undefined behaviour.
    ///
    /// # Example
    /// ```
    /// use math_engine::utils::untyped::Untyped;
    ///
    /// let value = 97;
    /// let p = Untyped::new(&value);
    /// assert_eq!(b'a', unsafe { *p.cast::<u8>()});
    /// ```
    #[inline]
    pub unsafe fn cast<T>(&self) -> &T{
        &*(self.pointer as *const T)
    }

    /// Cast this pointer to the given type and get a mutable reference to it.
    ///
    /// # Remarks
    /// If the target type size is greater than the pointed value the result is undefined behaviour.
    ///
    /// # Example
    /// ```
    /// use math_engine::utils::untyped::Untyped;
    ///
    /// let value = 97;
    /// let mut  p = Untyped::new(&value);
    /// unsafe { *p.cast_mut() = 10 };
    /// assert_eq!(10, value);
    /// ```
    #[inline]
    pub unsafe fn cast_mut<T>(&mut self) -> &mut T{
        &mut *(self.pointer as *mut T)
    }
}

impl <'a, T> From<&'a T> for Untyped<'a>{
    #[inline]
    fn from(reference: &'a T) -> Self {
        Untyped::new(reference)
    }
}

impl <'a, T> From<&'a mut T> for Untyped<'a>{
    #[inline]
    fn from(reference: &'a mut T) -> Self {
        Untyped::new(reference)
    }
}

impl <'a> Pointer for Untyped<'a>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.pointer, f)
    }
}

impl <'a> Debug for Untyped<'a>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.pointer, f)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::mem::MaybeUninit;

    #[test]
    fn cast_test(){
        let n = 10;
        let mut ptr = Untyped::new(&n);

        unsafe{
            assert_eq!(*ptr.cast::<i32>(), 10);

            *ptr.cast_mut::<i32>() = 25;
            assert_eq!(*ptr.cast::<i32>(), 25);
        }
    }

    #[test]
    fn cast_test2(){
        let mut p = Untyped::new(&97_u8);
        assert_eq!(b'a', unsafe { *p.cast::<u8>()});
    }

    #[test]
    fn new_unchecked_test(){
        let n = MaybeUninit::new(26).as_mut_ptr();
        let ptr = unsafe { Untyped::new_unchecked(n) };

        unsafe{
            assert_eq!(*ptr.cast::<i32>(), 26);
        }
    }

    #[test]
    fn into_raw_test(){
        let ptr = Untyped::new(&10);
        let p = ptr.into_raw();

        unsafe {
            let value = p as *const i32;
            assert_eq!(*value, 10);
        }
    }
}