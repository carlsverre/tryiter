<h1 align="center">tryiter</h1>
<p align="center">
  <a href="https://docs.rs/tryiter">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/tryiter">
  </a>
  <a href="https://crates.io/crates/tryiter">
    <img alt="crates.io" src="https://img.shields.io/crates/v/tryiter.svg">
  </a>
  <a href="https://github.com/carlsverre/tryiter/actions">
    <img alt="Build Status" src="https://github.com/carlsverre/tryiter/actions/workflows/rust.yml/badge.svg">
  </a>
</p>


Utility functions for Iterators of Results. This crate is heavily inspired by [TryStreamExt] in the [futures] crate.

[futures]: https://docs.rs/futures
[TryStreamExt]: https://docs.rs/futures/latest/futures/stream/trait.TryStreamExt.html

## `TryIteratorExt`

This crate exports a trait called `TryIteratorExt` which provides utility methods on top of any Iterator which returns a `Result`. `TryIteratorExt` is automatically implemented on top of compatible Iterators via a generic `impl` so all you have to do is import the trait and start calling methods on your iterators.

**Example:**
```rust
use tryiter::TryIteratorExt;

let iter = vec![Ok(1), Ok(2), Ok(3), Err("error")].into_iter();
let mut evens = iter.try_filter(|x| x % 2 == 0);

assert_eq!(evens.next(), Some(Ok(2)));
assert_eq!(evens.next(), Some(Err("error")));
```
