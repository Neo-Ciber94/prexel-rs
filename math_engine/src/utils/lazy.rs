use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, UnsafeCell};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::Once;

/// Provide a way for lazily initialize a value.
///
/// # Example
/// ```
/// use math_engine::utils::lazy::Lazy;
///
/// static GLOBAL : Lazy<Vec<u32>> = Lazy::new(||{
///     let mut temp = Vec::new();
///     temp.push(2);
///     temp.push(3);
///     temp.push(1);
///     temp
/// });
///
/// for n in GLOBAL.get(){
///     println!("{}", n);
/// }
/// ```
pub struct Lazy<T, F = fn() -> T> {
    /// Holds the value.
    value: UnsafeCell<Option<T>>,
    /// Provides the value to initialize this instance.
    initializer: Cell<Option<F>>,
    /// Used for a sync initialization.
    once: Once,
}

impl<T, F> Lazy<T, F> {
    /// Creates a new `Lazy<T>` where `init` provides the value of the instance.
    ///
    /// # Remarks
    /// - `F` is expected to be a `Fn() -> T`.
    ///
    /// # Examples
    /// ```
    /// use math_engine::utils::lazy::Lazy;
    /// let fruits = Lazy::new(|| ["Apple", "Orange", "Grape"]);
    /// assert_eq!(&["Apple", "Orange", "Grape"], fruits.get());
    /// ```
    #[inline]
    pub const fn new(initializer: F) -> Self {
        Lazy {
            value: UnsafeCell::new(None),
            initializer: Cell::new(Some(initializer)),
            once: Once::new(),
        }
    }

    /// Checks whether this instance is initialized.
    ///
    /// # Remarks
    /// - Returns `true` if the instance is initialized, otherwise false.
    #[inline]
    pub fn is_initialized(&self) -> bool {
        unsafe { (*self.value.get()).is_some() }
    }

    /// Sets the value of this instance.
    ///
    /// # Remarks
    /// - This may override the value of the lazy if is already initialized.
    ///
    /// # Examples
    /// ```
    /// use math_engine::utils::lazy::Lazy;
    ///
    /// let mut number = Lazy::new(|| 10);
    /// assert_eq!(number.get(), &10);
    ///
    /// number.set(42);
    /// assert_eq!(number.get(), &42)
    /// ```
    #[inline]
    pub fn set(&mut self, value: T) {
        unsafe { self.value.get().write(Some(value)) }
    }
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    /// Gets or creates a reference to the value of this instance.
    ///
    /// # Examples
    /// ```
    /// use math_engine::utils::lazy::Lazy;
    ///
    /// let vec = Lazy::new(|| 10);
    /// assert_eq!(*vec.get(), 10);
    /// ```
    #[inline]
    pub fn get(&self) -> &T {
        unsafe {
            match *self.value.get() {
                None => self.initialize(),
                Some(ref n) => n,
            }
        }
    }

    /// Gets or creates a mutable reference to the value of this instance.
    ///
    /// # Examples
    /// ```
    /// use math_engine::utils::lazy::Lazy;
    ///
    /// let mut vec = Lazy::new(|| 10);
    /// *vec.get_mut() = 30;
    /// assert_eq!(*vec.get(), 30);
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe {
            match *self.value.get() {
                None => self.initialize(),
                Some(ref mut n) => n,
            }
        }
    }

    /// Initialize this instance and gets the value.
    #[inline]
    unsafe fn initialize(&self) -> &mut T {
        let ptr = self.value.get();

        self.once.call_once(|| {
            let init = self
                .initializer
                .take()
                .expect("Lazy is already initialized");
            let value = init();
            ptr.write(Some(value));
        });

        (*ptr).as_mut().unwrap()
    }
}

impl<T: Default> Default for Lazy<T> {
    #[inline]
    fn default() -> Self {
        Lazy::new(T::default)
    }
}

unsafe impl<T, F> Sync for Lazy<T, F> {}

unsafe impl<T, F> Send for Lazy<T, F> {}

impl<T: Clone, F: FnOnce() -> T> Lazy<T, F> {
    /// Gets a copy of the inner value.
    ///
    /// # Examples
    /// ```
    /// use math_engine::utils::lazy::Lazy;
    ///
    /// let number = Lazy::new(|| 22);
    /// assert_eq!(number.clone_inner(), 22);
    /// ```
    #[inline]
    pub fn clone_inner(&self) -> T {
        self.get().clone()
    }
}

impl<T: Debug, F> Debug for Lazy<T, F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match &*self.value.get() {
                Some(ref n) => write!(f, "Lazy({:?})", n),
                None => write!(f, "Lazy(Uninitialized)"),
            }
        }
    }
}

impl<T: Display, F> Display for Lazy<T, F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match &*self.value.get() {
                Some(ref n) => write!(f, "Lazy({})", n),
                None => write!(f, "Lazy(Uninitialized)"),
            }
        }
    }
}

impl<T, F: FnOnce() -> T> Borrow<T> for Lazy<T, F> {
    #[inline]
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T, F: FnOnce() -> T> BorrowMut<T> for Lazy<T, F> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T, F: FnOnce() -> T> Deref for Lazy<T, F> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T, F: FnOnce() -> T> DerefMut for Lazy<T, F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn lazy_test0() {
        static GLOBAL: Lazy<Vec<u32>> = Lazy::new(|| {
            let mut temp = Vec::new();
            temp.push(3);
            temp.push(1);
            temp.push(2);
            temp
        });

        assert!(!GLOBAL.is_initialized());
        assert_eq!(&[3, 1, 2], GLOBAL.get().as_slice());
        assert_eq!(&[3, 1, 2], GLOBAL.get().as_slice());
        assert_eq!(&[3, 1, 2], GLOBAL.get().as_slice());
        assert!(GLOBAL.is_initialized());
    }

    #[test]
    fn lazy_test1() {
        static GLOBAL: Lazy<Vec<u32>> = Lazy::new(|| {
            let mut temp = Vec::new();
            temp.push(3);
            temp.push(1);
            temp.push(2);
            temp
        });

        assert!(!GLOBAL.is_initialized());
        assert_eq!(&[3, 1, 2], GLOBAL.get().as_slice());
        assert!(GLOBAL.is_initialized());
    }

    #[test]
    fn lazy_test2() {
        static mut GLOBAL: Lazy<Vec<u32>> = Lazy::new(|| {
            let mut temp = Vec::new();
            temp.push(3);
            temp.push(1);
            temp.push(2);
            temp
        });

        unsafe {
            let vec = GLOBAL.get_mut();
            vec.push(5);
            vec.push(4);
            vec.sort();
            assert_eq!(&[1, 2, 3, 4, 5], vec.as_slice());
        }
    }

    #[test]
    fn lazy_test4() {
        let fruits = Lazy::new(|| ["Apple", "Orange", "Grape"]);
        assert_eq!(&["Apple", "Orange", "Grape"], fruits.get());
    }

    #[test]
    fn lazy_set_test() {
        let mut number = Lazy::new(|| 22);
        assert_eq!(*number.get(), 22);

        number.set(10);
        assert_eq!(*number.get(), 10);
    }

    #[test]
    fn lazy_deref_test() {
        let mut numbers: Lazy<Vec<u32>> = Lazy::new(|| {
            let mut temp: Vec<u32> = Vec::new();
            temp.push(1);
            temp.push(2);
            temp
        });

        numbers.push(3);
        assert_eq!(&[1, 2, 3], numbers.as_slice());
    }

    #[test]
    fn lazy_default_test() {
        let number: Lazy<u32> = Lazy::default();
        assert_eq!(number.clone_inner(), 0);
    }

    //#[test]
    #[allow(dead_code)]
    fn multithreaded_access_test() {
        //static mut LAZY : Lazy<Mutex<Vec<u64>>> = Lazy::new(|| Mutex::new(Vec::new()));
        static mut LAZY: Lazy<Vec<u64>> = Lazy::new(|| Vec::new());

        let iterations = 10;
        unsafe {
            for x in 0..iterations {
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(10));
                    //LAZY.lock().unwrap().push(x);
                    LAZY.push(x);
                });
            }
        }
    }
}
