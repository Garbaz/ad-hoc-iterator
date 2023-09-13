#![no_std]

//! # Ad hoc Iterator
//!
//! This is a very small crate providing a macro and a function that allow for
//! conveniently creating iterators on the fly.
//! 
//! With the [`iterate`](iterate) macro of this crate, you can however do
//! exactly that. See it's documentation for more information.
//!
//! With the [`iterator_from`](iterator_from) function, you can directly create
//! an iterator from an `FnMut` closure (which is exactly the same as what the
//! `iterate` macro is doing behind the scenes).

/// Internal, not intended for direct use.
///
/// See [`iterate`](iterate) & [`iterator_from`](iterator_from).
#[doc(hidden)]
pub struct __ClosureIterator<F>(pub Option<F>);

impl<T, F: FnMut() -> Option<T>> Iterator for __ClosureIterator<F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.0.as_mut().and_then(|f| f());
        if r.is_none() {
            self.0 = None;
            None
        } else {
            r
        }
    }
}

/// Internal, not intended for direct use.
///
/// See [`iterate`](iterate) & [`iterator_from`](iterator_from).
#[doc(hidden)]
#[macro_export]
macro_rules! __iterate {
    ($b:block) => {
        ::ad_hoc_iterator::iterator_from(move || $b)
    };
}

/// Create an ad hoc iterator.
///
/// # Usage
///
/// The macro is used just like defining a closure. The return type of it's body
/// has to be `Option<T>` for some type T. So at the minimum: `iterate!{ None }`
///
/// The expression `iterate! {...}` is of type `impl Iterator<T>`.
///
/// Any captured variables are moved (like with `move || {...}` closures).
///
/// You can use `return` statements in the body of `iterate!`.
///
/// # Example
///
/// ```
/// use ad_hoc_iterator::iterate;
///
/// fn count_to(n: usize) -> impl Iterator<Item = usize> {
///     let mut i = 0;
///     iterate! {
///         if i < n {
///             i += 1;
///             Some(i-1)
///         } else {
///             None
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! iterate {
    ($( $ts:tt )*) => {
        ::ad_hoc_iterator::__iterate!{{ $($ts)* }}
    };
}

/// Turn a closure into an iterator.
///
/// Each `next()` on the iterator will simply call the closure once. The
/// iterator ends when the closure returns `None`. The closure will not be
/// called again after that point, even if `next()` is called again.
///
/// # Example
///
/// ```
/// use ad_hoc_iterator::iterator_from;
///
/// fn count_from_to(n: usize, m: usize) -> impl Iterator<Item = usize> {
///     let mut i = n;
///     iterator_from(move || {
///         if i < m {
///             i += 1;
///             Some(i - 1)
///         } else {
///             None
///         }
///     })
/// }
/// ```
#[inline(always)]
pub fn iterator_from<T, F: FnMut() -> Option<T>>(f: F) -> impl Iterator<Item = T> {
    __ClosureIterator(Some(f))
}
