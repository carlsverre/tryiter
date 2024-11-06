# tryiter
Utility functions for Iterators of Results. This crate is heavily inspired by [TryStreamExt] in the [futures] crate.

[futures]: https://docs.rs/futures
[TryStreamExt]: https://docs.rs/futures/latest/futures/stream/trait.TryStreamExt.html

## `TryIteratorExt`

This crate exports a trait called `TryIteratorExt` which provides utility methods on top of any Iterator which returns a `Result`. `TryIteratorExt` is automatically implemented on top of compatible Iterators via a generic `impl` so all you have to do is import the trait and start calling methods on your iterators.
