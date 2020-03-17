use std::marker::PhantomData;
use std::fmt::{Pointer, Formatter, Debug};

/// Represents a pointer with not type, equivalent to C `void*`.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Untyped<'a>{
    pointer: *const (),
    _marker: &'a PhantomData<()>
}

impl <'a> Untyped<'a>{
    /// Creates a new pointer to the given reference.
    #[inline]
    pub const fn new<T>(value: &'a T) -> Untyped<'a>{
        let raw = value as *const _ as *const ();
        Untyped {
            pointer: raw,
            _marker: &PhantomData
        }
    }

    /// Creates a pointer using the given.
    #[inline]
    pub const unsafe fn new_unchecked<T>(ptr: *mut T) -> Untyped<'a>{
        Untyped {
            pointer: ptr as _,
            _marker: &PhantomData
        }
    }

    /// Converts this untyped to a raw pointer.
    #[inline]
    pub fn into_raw(self) -> *const (){
        self.pointer
    }

    /// Cast this pointer to the given type and get a reference to it.
    #[inline]
    pub unsafe fn cast<T>(&self) -> &T{
        &*(self.pointer as *const T)
    }

    /// Cast this pointer to the given type and get a mutable reference to it.
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