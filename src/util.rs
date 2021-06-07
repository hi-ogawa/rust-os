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
    (($($vis:tt)*) $N:ident : $T:ty = $e:expr) => {
        use core::ops::Deref;
        use core::ops::DerefMut;
        use $crate::util::Once;
        use paste::paste;

        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        $($vis)* struct $N { __private_field: () }
        $($vis)* static mut $N: $N = $N { __private_field: () };

        paste! {
            static mut [<ONCE__ $N>]: Once<$T> = Once::INIT;
        }

        impl Deref for $N {
            type Target = $T;

            fn deref(&self) -> &Self::Target {
                paste! {
                    unsafe { [<ONCE__ $N>].call_once(|| { $e }) }
                }
            }
        }

        impl DerefMut for $N {
            fn deref_mut(&mut self) -> &mut Self::Target {
                paste! {
                    unsafe { [<ONCE__ $N>].call_once(|| { $e }) }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! lazy_static {
    (static mut $N:ident : $T:ty = $e:expr;) => {
        $crate::lazy_static_impl!(() $N : $T = $e);
    };

    (pub static mut $N:ident : $T:ty = $e:expr;) => {
        $crate::lazy_static_impl!((pub) $N : $T = $e);
    }
}
