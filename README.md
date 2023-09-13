# Ad-hoc Iterator

‖ [__Docs.rs__](https://docs.rs/ad-hoc-iterator/latest/ad-hoc-iterator/) ‖ [__Lib.rs__](https://lib.rs/crates/ad-hoc-iterator) ‖ [__Crates.io__](https://crates.io/crates/ad-hoc-iterator/) ‖

This is a very small crate providing a macro and a function that allow for conveniently creating iterators on the fly.
 
The `Iterator` Trait is very useful. The problem ist just that we can't simply construct an iterator in-place, but rather have to define a struct, `impl` the `Iterator` trait for it, and then return a value of that struct.
 
With the [`iterate`](iterate) macro of this crate, you can however do exactly that. See it's documentation for more information.
 
With the [`iterator_from`](iterator_from) function, you can directly create an iterator from an `FnMut` closure (which is exactly the same as what the `iterate` macro is doing).