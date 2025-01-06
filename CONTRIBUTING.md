# Contributions guide

Please follow this guide when contributing new features to this library.

## Accepted Contributions

This crate is currently only accepting new functionality added to the `TryIteratorExt` trait which is aligned with existing functions on any of the following third party types:

- std::iter::[Iterator]
- futures::stream::[TryStreamExt]
- itertools::[Itertools]

A good list of missing adapters can be found in Yoshua Wuyts post on [Fallible Iterator Adapters].

[Iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[TryStreamExt]: https://docs.rs/futures/latest/futures/stream/trait.TryStreamExt.html
[Itertools]: https://docs.rs/itertools/latest/itertools/trait.Itertools.html
[Fallible Iterator Adapters]:https://blog.yoshuawuyts.com/fallible-iterator-adapters/

## Testing

Functions should be sufficiently simple that they can be verified via inspection along with a simple doctest which also serves as documentation.

If the functionality is complex consider adding additional secondary tests.

## Submitting a pull request

Before submitting a PR please run `cargo test` and `cargo check` fixing any errors that come up. Thank you for your contributions!