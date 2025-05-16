//! Trivial "Box" data structure.
//!
//! This data structure was originally called "MyNumber" in the course,
//! and contains no functionality beyond storing a single value (a number, in the tests).
//! For this implementation, it has been expanded to cover all reasonable `T`.

use core::fmt;

pub use implementation::MyBox;

/// Unsafe-restraining module.
#[allow(unsafe_code)]
mod implementation {
    use core::{
        alloc::Layout,
        mem::ManuallyDrop,
        ops::{Deref, DerefMut},
        ptr::NonNull,
    };

    /// Simple heap allocation of a single value.
    ///
    /// Like [`alloc::boxed::Box`], this type stores a single `T` on the heap.
    #[doc(alias = "MyNumber")]
    pub struct MyBox<T> {
        // critical invariant: `inner` must be a valid pointer to a valid T, and if T is not zero-sized it must be possible to dealloc it
        inner: NonNull<T>,
    }

    impl<T> MyBox<T> {
        const INNER_LAYOUT: Layout = Layout::new::<T>();

        /// Place the given `value` on the heap.
        #[inline]
        pub fn new(value: T) -> Self {
            // find the correct size+alignment requirements for this value
            if Self::INNER_LAYOUT.size() == 0 {
                // special case: 0-sized types can not be safely allocated!
                return Self {
                    inner: NonNull::dangling(),
                };
            }
            // SAFETY: T is not zero-size
            let raw = unsafe { alloc::alloc::alloc(Self::INNER_LAYOUT) };
            // convert the pointer into the proper pointer type
            let inner = match NonNull::new(raw) {
                Some(u8_inner) => u8_inner.cast::<T>(),
                None => alloc::alloc::handle_alloc_error(Self::INNER_LAYOUT),
            };
            // write the value into the new allocation
            // SAFETY: `inner` has no outstanding references and is of the proper layout
            unsafe { inner.write(value) };

            // we now have a properly-initialized value!
            Self { inner }
        }

        /// Remove the value from the heap and return it, deallocating the box.
        #[inline]
        pub fn into_inner(self) -> T {
            // ensure that `self` is never dropped, and so the original destructor can never accidentally run after this call
            let manually_drop = ManuallyDrop::new(self);
            let inner = manually_drop.inner;
            // move the value from the heap to the stack
            /*
            SAFETY:
            - *self.inner is valid (no API provided can move out of it or change the pointer, and `new` ensured it was valid)
            - self.inner will never be accessed as T after this (destructor was explicitly prevented above for this reason)
            */
            let value: T = unsafe { inner.read() };

            if Self::INNER_LAYOUT.size() == 0 {
                // special case: can't, and don't need to, free 0-size values
                return value;
            }

            // get a de-allocatable pointer to the heap allocation
            let raw_inner = inner.as_ptr().cast::<u8>();
            // deallocate the heap pointer
            // SAFETY: pointer is unchanged from the pointer returned by `alloc`, and the layout is identical, fulfilling `dealloc`'s requirements
            unsafe { alloc::alloc::dealloc(raw_inner, Self::INNER_LAYOUT) };

            // return the inner value
            value
        }
    }

    /// Translation of requirement to be able to read the value.
    impl<T> Deref for MyBox<T> {
        type Target = T;

        #[doc(alias = "read")]
        #[inline]
        fn deref(&self) -> &T {
            // SAFETY: `inner` is always valid as a reference to a T
            unsafe { self.inner.as_ref() }
        }
    }

    /// Translation of requirement to be able to overwrite the value.
    impl<T> DerefMut for MyBox<T> {
        #[doc(alias = "write")]
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            // SAFETY: `inner` is valid as a reference to a T, and the caller has an exclusive reference to this `MyBox<T>`
            unsafe { self.inner.as_mut() }
        }
    }

    /// Translation of destructor.
    impl<T> Drop for MyBox<T> {
        #[inline]
        fn drop(&mut self) {
            // allow the inner resource to free its own resources, if it has any
            // SAFETY: `inner` is valid until this line, and `drop` is the last function to ever be called on `self` (including not calling `drop` again)
            unsafe { self.inner.drop_in_place() };

            if Self::INNER_LAYOUT.size() == 0 {
                // special case: ZSTs do not allocate, and can not be deallocated
                return;
            }

            // free the internal allocation
            let raw_inner = self.inner.as_ptr().cast::<u8>();
            // SAFETY: `inner` was valid as an allocation until this line, the layout matches, and T is not a ZST
            unsafe { alloc::alloc::dealloc(raw_inner, Self::INNER_LAYOUT) };
        }
    }
}

/// Rust-specific helper to visualize this type in a programmer-friendly way.
impl<T: fmt::Debug> fmt::Debug for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MyBox").field(self as &T).finish()
    }
}

/// Translation of the copy constructor into Rust.
impl<T: Clone> Clone for MyBox<T> {
    fn clone(&self) -> Self {
        Self::new(T::clone(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_numbers() {
        for i in [1, 2, 3] {
            let boxed: MyBox<i32> = MyBox::new(i);
            std::println!("my box stores {n}", n = *boxed);
        }
    }

    #[test]
    fn number_into_number() {
        for i in [1, 2, 3] {
            let boxed: MyBox<i32> = MyBox::new(i);
            std::println!("my box unwraps into {n}", n = boxed.into_inner());
            // boxed should not be double-freed
        }
    }

    #[test]
    fn store_unit_into_inner() {
        // make sure we can handle zero-sized types correctly
        let boxed: MyBox<()> = MyBox::new(());
        std::println!("the box contains {unit:?}", unit = *boxed);
        std::println!("the box unwraps into {unit:?}", unit = boxed.into_inner());
    }

    #[test]
    fn store_unit_drop() {
        // make sure we can handle zero-sized types correctly
        let boxed: MyBox<()> = MyBox::new(());
        std::println!("the box contains {unit:?}", unit = *boxed);
        core::mem::drop(boxed);
        // should not double-free
    }
}
