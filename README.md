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


Utility functions for Iterators of Results. This crate is heavily inspired by [TryStreamExt] in the [futures] crate as well as Yoshua Wuyts post on [Fallible Iterator Adapters].

[futures]: https://docs.rs/futures
[TryStreamExt]: https://docs.rs/futures/latest/futures/stream/trait.TryStreamExt.html
[Fallible Iterator Adapters]:https://blog.yoshuawuyts.com/fallible-iterator-adapters/

## `TryIteratorExt`

This crate exports a trait called `TryIteratorExt` which provides utility methods on top of any Iterator which returns a `Result`. `TryIteratorExt` is automatically implemented on top of compatible Iterators via a generic `impl` so all you have to do is import the trait and start calling methods on your iterators.

The methods provide two fundamental simplifications over regular Iterator methods:

1. They operate directly on `Ok` or `Err` values, passing through the other values transparently.
2. They take in fallible closures which can return `Err`.

See [tests/sanity.rs](./tests/sanity.rs) for a quick overview of using this crate.

## Release Stability

This crate is still pre-1.0 and will be until someone wants to use it in production. If you want to use it before that point please pin a specific version number and be prepared for breaking changes.