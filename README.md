# Ad hoc Iterator

‖ [__Docs.rs__](https://docs.rs/ad-hoc-iterator/latest/ad-hoc-iterator/) ‖ [__Lib.rs__](https://lib.rs/crates/ad-hoc-iterator) ‖ [__Crates.io__](https://crates.io/crates/ad-hoc-iterator/) ‖

The `Iterator` Trait is very useful. The problem ist just that we can't simply construct an iterator in-place, but rather have to define a struct, `impl` the `Iterator` trait for it, and then return a value of that struct. This crates exists to alleviate that inconvenience.