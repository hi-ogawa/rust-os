//
// Owned array iterator
//

#[derive(Debug, Copy, Clone)]
pub struct OwnedArrayIterator<T: Sized + Copy, const N: usize> {
    inner: [T; N],
    index: usize,
}

impl<T: Sized + Copy, const N: usize> OwnedArrayIterator<T, N> {
    pub fn new(inner: [T; N]) -> Self {
        Self { inner, index: 0 }
    }
}

impl<T: Sized + Copy, const N: usize> Iterator for OwnedArrayIterator<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < N {
            let item = self.inner[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

//
// read_volatile/write_volatile
//
#[repr(C)]
pub struct Volatile<T>(T);

impl<T: Copy> Volatile<T> {
    fn as_ptr(&self) -> *const T {
        &self.0 as *const T
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.0 as *mut T
    }

    pub fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(self.as_ptr()) }
    }

    pub fn write(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(self.as_mut_ptr(), value) }
    }
}

//
// reinterpret cast
//

pub unsafe fn address_cast<'a, T>(address: usize) -> &'a T {
    &*(address as *const T)
}

pub unsafe fn address_cast_mut<'a, T>(address: usize) -> &'a mut T {
    &mut *(address as *mut T)
}

pub unsafe fn reinterpret_cast<'a, T1, T2>(reference: &T1) -> &'a T2 {
    &*((reference as *const T1) as *const T2)
}

pub unsafe fn reinterpret_cast_mut<'a, T1, T2>(reference: &mut T1) -> &'a mut T2 {
    &mut *((reference as *mut T1) as *mut T2)
}

//
// Fake Mutex to use static nicely
//

pub struct Mutex<T>(T);

impl<T> Mutex<T> {
    pub const fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn lock<'a>(&self) -> &'a mut T {
        unsafe { &mut *(&self.0 as *const T as *mut T) }
    }
}

//
// Simple but unsafe lazy_static
//

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

pub struct Once<T> {
    data: UnsafeCell<MaybeUninit<T>>,
    done: bool,
}

impl<T> Once<T> {
    pub const INIT: Self = Self {
        data: UnsafeCell::new(MaybeUninit::uninit()),
        done: false,
    };

    pub fn call_once<F: Fn() -> T>(&mut self, f: F) -> &mut T {
        if !self.done {
            unsafe {
                (*self.data.get()).as_mut_ptr().write(f());
            }
            self.done = true;
        }
        unsafe { &mut *(*self.data.get()).as_mut_ptr() }
    }
}

#[macro_export]
macro_rules! lazy_static_impl {
    (($($vis:tt)?) $N:ident : $T:ty = $e:expr) => {
        use core::ops::Deref;
        use $crate::util::Once;

        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        $($vis)? struct $N { __private_field: () }
        $($vis)? static $N: $N = $N { __private_field: () };

        impl Deref for $N {
            type Target = $T;

            fn deref(&self) -> &Self::Target {
                static mut ONCE: Once<$T> = Once::INIT;
                let closure = || { $e };
                unsafe { ONCE.call_once(closure) }
            }
        }
    };
}

#[macro_export]
macro_rules! lazy_static {
    (static ref $N:ident : $T:ty = $e:expr;) => {
        $crate::lazy_static_impl!(() $N : $T = $e);
    };

    (pub static ref $N:ident : $T:ty = $e:expr;) => {
        $crate::lazy_static_impl!((pub) $N : $T = $e);
    }
}
