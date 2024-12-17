use std::iter::FusedIterator;

use crate::TryIterator;

/// An iterator with a `try_peek()` that returns an optional reference to the next
/// Ok value while forwarding errors. Errors are not stored by `try_peek()`
/// such that subsequent calls to `try_peek()` will continue to consume the
/// underlying Iterator until they reach an `I::Ok` value.
///
/// This `struct` is created by the [`try_peekable`] method on [`TryIteratorExt`]. See its
/// documentation for more.
///
/// The [`Iterator`] implementation for TryPeekable is copied from the standard
/// libraries implementation of [`Peekable`] with modifications due to only
/// peeking Ok values.
///
/// [`try_peekable`]: crate::TryIteratorExt::try_peekable
/// [`TryIteratorExt`]: crate::TryIteratorExt
/// [`Peekable`]: core::iter::Peekable
/// [`Iterator`]: core::iter::Iterator
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct TryPeekable<I: TryIterator> {
    iter: I,
    /// Remember a peeked value, even if it was `None`.
    peeked: Option<Option<I::Ok>>,
}

impl<I: TryIterator> TryPeekable<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter, peeked: None }
    }

    pub fn try_peek(&mut self) -> Result<Option<&I::Ok>, I::Err> {
        match self.peeked {
            Some(ref v) => Ok(v.as_ref()),
            None => match self.iter.next() {
                Some(Ok(v)) => Ok(self.peeked.insert(Some(v)).as_ref()),
                Some(Err(e)) => Err(e),
                None => {
                    self.peeked = Some(None);
                    Ok(None)
                }
            },
        }
    }

    pub fn try_peek_mut(&mut self) -> Result<Option<&mut I::Ok>, I::Err> {
        match self.peeked {
            Some(ref mut v) => Ok(v.as_mut()),
            None => match self.iter.next() {
                Some(Ok(v)) => Ok(self.peeked.insert(Some(v)).as_mut()),
                Some(Err(e)) => Err(e),
                None => {
                    self.peeked = Some(None);
                    Ok(None)
                }
            },
        }
    }
}

impl<I: TryIterator + ExactSizeIterator> ExactSizeIterator for TryPeekable<I> {}
impl<I: TryIterator + FusedIterator> FusedIterator for TryPeekable<I> {}

impl<I: TryIterator> Iterator for TryPeekable<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        match self.peeked.take() {
            Some(Some(peeked)) => Some(Ok(peeked)),
            Some(None) => None,
            None => self.iter.next(),
        }
    }

    #[inline]
    fn count(mut self) -> usize {
        match self.peeked.take() {
            Some(None) => 0,
            Some(Some(_)) => 1 + self.iter.count(),
            None => self.iter.count(),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        match self.peeked.take() {
            Some(None) => None,
            Some(Some(v)) if n == 0 => Some(Ok(v)),
            Some(Some(_)) => self.iter.nth(n - 1),
            None => self.iter.nth(n),
        }
    }

    #[inline]
    fn last(mut self) -> Option<I::Item> {
        let peek_opt = match self.peeked.take() {
            Some(None) => return None,
            Some(Some(v)) => Some(Ok(v)),
            None => None,
        };
        self.iter.last().or(peek_opt)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let peek_len = match self.peeked {
            Some(None) => return (0, Some(0)),
            Some(Some(_)) => 1,
            None => 0,
        };
        let (lo, hi) = self.iter.size_hint();
        let lo = lo.saturating_add(peek_len);
        let hi = match hi {
            Some(x) => x.checked_add(peek_len),
            None => None,
        };
        (lo, hi)
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let acc = match self.peeked {
            Some(None) => return init,
            Some(Some(v)) => fold(init, Ok(v)),
            None => init,
        };
        self.iter.fold(acc, fold)
    }
}
