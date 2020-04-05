use std::any::TypeId;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::sync::Once;

/// A cache for store static data.
///
/// # Remarks
/// The main proporse of this `struct` is allow generic static local variables.
///
/// # Examples
/// ```
/// use math_engine::utils::static_store::StaticStore;
///
/// let store = StaticStore::new();
/// let x = store.load(|| 10);
///
/// assert_eq!(*x, 10);
/// assert_eq!(*store.get::<i32>(), 10);
/// ```
/// Or using static local variables
/// ```
/// use math_engine::utils::static_store::StaticStore;
///
/// fn add<T: 'static>(value: T) -> usize{
///     static mut STORE : StaticStore = StaticStore::new();
///
///     let vec = unsafe { STORE.load_mut(|| Vec::new()) };
///     vec.push(value);
///     vec.len()
/// }
///
/// assert_eq!(add(10), 1);
/// assert_eq!(add(20), 2);
/// assert_eq!(add("Hello"), 1);
/// ```
pub struct StaticStore<C = HashMap<TypeId, *const ()>> {
    /// Holds the `HashMap` to store the data.
    store: UnsafeCell<Option<C>>,
    /// Used for a thread safe initialization of the `store`.
    once: Once
}

impl StaticStore<HashMap<TypeId, *const ()>> {
    /// Constructs a new `StaticStore`.
    #[inline]
    pub const fn new() -> Self{
        StaticStore{
            store: UnsafeCell::new(None),
            once: Once::new()
        }
    }

    /// Gets and/or initialize a reference to a value of the specified type.
    ///
    /// # Parameters
    /// - f: the function that provides the value to store.
    #[inline]
    pub fn load<T: 'static, F: FnOnce() -> T>(&self, f: F) -> &'static T{
        let map = Self::get_or_init(self);

        let raw = map.entry(TypeId::of::<T>())
            .or_insert_with(|| Box::into_raw(Box::new(f())) as *const ())
            .clone();

        unsafe { &*(raw as *const T) }
    }

    /// Gets and/or initialize a mutable reference to a value of the specified type.
    ///
    /// # Parameters
    /// - f: the function that provides the value to store.
    #[inline]
    pub fn load_mut<T: 'static, F: FnOnce() -> T>(&mut self, f: F) -> &'static mut T{
        let map = Self::get_or_init(self);
        let raw = map.entry(TypeId::of::<T>())
            .or_insert_with(|| Box::into_raw(Box::new(f())) as *const ())
            .clone();

        unsafe { &mut *(raw as *mut T) }
    }

    /// Gets a reference of the data of the specified type.
    #[inline]
    pub fn get<T: 'static>(&self) -> Option<&'static T>{
        let map = Self::get_or_init(self);
        let raw = map.get(&TypeId::of::<T>());
        match raw {
            Some(ptr) => {
                unsafe { Some(&*(*ptr as *const T)) }
            },
            None => None
        }
    }

    /// Gets a mutable reference of the data of the specified type.
    #[inline]
    pub fn get_mut<T: 'static>(&mut self) -> Option<&'static mut T>{
        let map = Self::get_or_init(self);
        let raw = map.get(&TypeId::of::<T>());
        match raw {
            Some(ptr) => {
                unsafe { Some(&mut *(*ptr as *mut T)) }
            },
            None => None
        }
    }

    fn get_or_init(&self) -> &mut HashMap<TypeId, *const ()>{
        let raw = self.store.get();
        let data = unsafe { &mut *raw };

        match data {
            None => {
                unsafe{
                    self.once.call_once(||{
                        raw.write(Some(HashMap::new()))
                    });

                    (*raw).as_mut().unwrap()
                }
            },
            Some(ref mut map) => map,
        }
    }
}

unsafe impl<C> Sync for StaticStore<C>{}

unsafe impl<C> Send for StaticStore<C>{}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn load_test1(){
        let store = StaticStore::new();

        let x1 = store.load(|| 10);
        assert_eq!(*x1, 10);
        assert_eq!(*store.get::<i32>().unwrap(), 10);
    }

    #[test]
    fn load_test2(){
        let mut store = StaticStore::new();

        let x1 = store.load(|| 10);
        assert_eq!(*x1, 10);

        *store.get_mut::<i32>().unwrap() = 5;
        assert_eq!(*store.get::<i32>().unwrap(), 5);
    }

    #[test]
    fn load_mut_test(){
        let mut store = StaticStore::new();

        let vec = store.load_mut(|| Vec::<i32>::new());
        vec.push(10);
        vec.push(20);
        vec.push(30);

        assert_eq!(vec.as_slice(), &[10, 20, 30]);
        assert_eq!(store.get::<Vec<i32>>().unwrap().as_slice(), &[10, 20, 30]);
    }

    #[test]
    fn thread_local_test(){
        thread_local! { static STORE : StaticStore = StaticStore::new() }

        STORE.with(|n| {
            let x1 = n.load(|| "Hello");
            assert_eq!(*x1, "Hello");
            assert_eq!(*n.get::<&str>().unwrap(), "Hello");
        });
    }

    #[test]
    fn static_store_test(){
        assert_eq!(add(10), 1);
        assert_eq!(add(20), 2);

        assert_eq!(add("mom"), 1);
        assert_eq!(add("dad"), 2);

        assert_eq!(add('a'), 1);
        assert_eq!(add(20), 3);
    }

    fn add<T: 'static>(value: T) -> usize{
        static mut STORE : StaticStore = StaticStore::new();

        let vec = unsafe { STORE.load_mut(|| Vec::new()) };
        vec.push(value);
        vec.len()
    }
}