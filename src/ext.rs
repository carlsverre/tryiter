use std::iter;

use crate::{TryIterator, TryPeekable};

pub trait TryIteratorExt: TryIterator {
    /// Attempt to retrieve the next value from the iterator, lifting the error
    /// if one occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let mut iter = vec![Ok(5), Err(5)].into_iter();
    ///
    /// assert_eq!(iter.try_next(), Ok(Some(5)));
    /// assert_eq!(iter.try_next(), Err(5));
    /// ```
    fn try_next(&mut self) -> Result<Option<Self::Ok>, Self::Err> {
        self.next().transpose()
    }

    /// Wraps the current iterator in a new iterator that converts the error
    /// type into the one provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let mut iter = vec![Ok(()), Err(5i32)].into_iter().err_into::<i64>();
    ///
    /// assert_eq!(iter.next(), Some(Ok(())));
    /// assert_eq!(iter.next(), Some(Err(5i64)));
    /// ```
    fn err_into<E>(self) -> impl TryIterator<Ok = Self::Ok, Err = E>
    where
        Self: Sized,
        Self::Err: Into<E>,
    {
        self.map(|result| result.map_err(Into::into))
    }

    /// Wraps the current iterator in a new iterator that maps the success value
    /// using the provided closure.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let mut iter = vec![Ok(5), Err(5)].into_iter().map_ok(|x| Ok(x * 2));
    ///
    /// assert_eq!(iter.next(), Some(Ok(10)));
    /// assert_eq!(iter.next(), Some(Err(5)));
    /// ```
    fn map_ok<T, F>(mut self, mut f: F) -> impl TryIterator<Ok = T, Err = Self::Err>
    where
        Self: Sized,
        F: FnMut(Self::Ok) -> Result<T, Self::Err>,
    {
        iter::from_fn(move || self.next().map(|result| result.and_then(&mut f)))
    }

    /// Wraps the current iterator in a new iterator that maps the error value
    /// using the provided closure.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let mut iter = vec![Ok(5), Err(5)].into_iter().map_err(|x| x * 2);
    ///
    /// assert_eq!(iter.next(), Some(Ok(5)));
    /// assert_eq!(iter.next(), Some(Err(10)));
    /// ```
    fn map_err<E, F>(mut self, mut f: F) -> impl TryIterator<Ok = Self::Ok, Err = E>
    where
        Self: Sized,
        F: FnMut(Self::Err) -> E,
    {
        iter::from_fn(move || self.next().map(|result| result.map_err(&mut f)))
    }

    /// Wraps the current iterator in a new iterator that filters and maps the
    /// success values using the provided closure. Errors are passed through.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let iter = vec![Ok(1), Ok(6), Err("error")].into_iter();
    /// let mut halves = iter.try_filter_map(|x| {
    ///     let ret = if x % 2 == 0 { Some(x / 2) } else { None };
    ///     Ok(ret)
    /// });
    ///
    /// assert_eq!(halves.next(), Some(Ok(3)));
    /// assert_eq!(halves.next(), Some(Err("error")));
    /// ```
    fn try_filter_map<T, F>(self, mut f: F) -> impl TryIterator<Ok = T, Err = Self::Err>
    where
        Self: Sized,
        F: FnMut(Self::Ok) -> Result<Option<T>, Self::Err>,
    {
        self.filter_map(move |result| match result {
            Ok(ok) => f(ok).transpose(),
            Err(err) => Some(Err(err)),
        })
    }

    /// Wraps the current iterator in a new iterator that filters the success
    /// values using the provided closure. Errors are passed through.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let iter = vec![Ok(1), Ok(2), Ok(3), Err("error")].into_iter();
    /// let mut evens = iter.try_filter(|x| Ok(x % 2 == 0));
    ///
    /// assert_eq!(evens.next(), Some(Ok(2)));
    /// assert_eq!(evens.next(), Some(Err("error")));
    /// ```
    fn try_filter<P>(self, mut predicate: P) -> impl TryIterator<Ok = Self::Ok, Err = Self::Err>
    where
        Self: Sized,
        P: FnMut(&Self::Ok) -> Result<bool, Self::Err>,
    {
        self.try_filter_map(move |value| {
            if predicate(&value)? {
                Ok(Some(value))
            } else {
                Ok(None)
            }
        })
    }

    /// Returns `true` if the provided closure returns `true` for all success
    /// values in the iterator. Errors are passed through.
    ///
    /// This method is short-circuiting; it will stop processing as soon as the
    /// closure returns `false`. This means that it may not visit all elements
    /// in the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let mut iter = vec![Ok::<i32, i32>(1), Ok(2), Ok(3)].into_iter();
    /// assert!(iter.try_all(|x| Ok(x < 4)).unwrap());
    ///
    /// let mut iter = vec![Ok(1), Ok(2), Err("error"), Ok(3)].into_iter();
    /// assert_eq!(iter.try_all(|x| Ok(x < 4)), Err("error"));
    ///
    /// let mut iter = vec![Ok(1), Ok(2), Err("error"), Ok(3)].into_iter();
    /// assert_eq!(iter.try_all(|x| Ok(x > 4)), Ok(false));
    /// ```
    ///
    /// Stopping at the first `false`:
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let mut iter = vec![Ok(1), Ok(2), Ok(3), Err("error")].into_iter();
    /// assert!(!iter.try_all(|x| Ok(x != 2)).unwrap());
    ///
    /// // The iterator stopped before consuming all elements
    /// assert_eq!(iter.next(), Some(Ok(3)));
    /// assert_eq!(iter.next(), Some(Err("error")));
    /// ```
    fn try_all<F>(&mut self, mut f: F) -> Result<bool, Self::Err>
    where
        Self: Sized,
        F: FnMut(Self::Ok) -> Result<bool, Self::Err>,
    {
        for result in self {
            if !result.and_then(&mut f)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Returns `true` if the provided closure returns `true` for any success
    /// values in the iterator. Errors are passed through.
    ///
    /// This method is short-circuiting; it will stop processing as soon as the
    /// closure returns `true`. This means that it may not visit all elements
    /// in the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let mut iter = vec![Ok(1), Ok(2), Ok(3), Err("error")].into_iter();
    /// assert!(iter.try_any(|x| Ok(x == 3)).unwrap());
    ///
    /// let mut iter = vec![Ok(1), Ok(2), Err("error"), Ok(3)].into_iter();
    /// assert_eq!(iter.try_any(|x| Ok(x == 3)), Err("error"));
    ///
    /// let mut iter = vec![Ok(1), Ok(2), Err("error"), Ok(3)].into_iter();
    /// assert_eq!(iter.try_any(|x| Ok(x != 1)), Ok(true));
    /// ```
    ///
    /// Stopping at the first `true`:
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let mut iter = vec![Ok(1), Ok(2), Ok(3), Err("error")].into_iter();
    /// assert!(iter.try_any(|x| Ok(x == 2)).unwrap());
    ///
    /// // The iterator stopped before consuming all elements
    /// assert_eq!(iter.next(), Some(Ok(3)));
    /// assert_eq!(iter.next(), Some(Err("error")));
    /// ```
    fn try_any<F>(&mut self, mut f: F) -> Result<bool, Self::Err>
    where
        Self: Sized,
        F: FnMut(Self::Ok) -> Result<bool, Self::Err>,
    {
        for result in self {
            if result.and_then(&mut f)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Wraps the current iterator in a new iterator that allows peeking at the
    /// next element without consuming it.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let mut iter = vec![Ok(1), Err("error"), Ok(2)].into_iter();
    /// let mut peek = iter.try_peekable();
    ///
    /// assert_eq!(peek.try_peek(), Ok(Some(&1)));
    ///
    /// // mutable references can also be acquired
    /// assert_eq!(peek.try_peek_mut(), Ok(Some(&mut 1)));
    ///
    /// assert_eq!(peek.try_next(), Ok(Some(1)));
    /// assert_eq!(peek.try_peek(), Err("error"));
    ///
    /// // Note that errors are not stored, subsequent calls to try_peek will
    /// // consume the next value from the iterator
    /// assert_eq!(peek.try_peek(), Ok(Some(&2)));
    /// assert_eq!(peek.try_peek(), Ok(Some(&2)));
    /// assert_eq!(peek.try_next(), Ok(Some(2)));
    ///
    /// // The iterator is now empty
    /// assert_eq!(peek.try_next(), Ok(None));
    ///
    /// ```
    fn try_peekable(self) -> TryPeekable<Self>
    where
        Self: Sized,
    {
        TryPeekable::new(self)
    }

    /// This is basically the fallible version of [`std::iter::Iterator::unzip`]
    ///
    /// Converts an iterator of [`Result`] of pairs into a [`Result`] of pair of containers.
    ///
    /// `try_unzip()` consumes the iterator, 
    /// - either until it encounters an error, in which case it would stop and return the error (short-circuiting).
    /// - or entirely if no error is encountered, producing a [`Result::Ok`] of two collections: one collection from the left elements of the pairs, and one from the right elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// let successful: Vec<Result<_, String>> = vec![Ok((1, 2)), Ok((3, 4)), Ok((5, 6))];
    /// let unzipped = successful.into_iter().try_unzip();
    /// let expected_left = vec![1, 3, 5];
    /// let expected_right = vec![2, 4, 6];
    /// assert_eq!(unzipped, Ok((expected_left,expected_right)));
    ///
    /// let errorneous = vec![Ok((1, 2)), Err("No number found"), Ok((5, 6))];
    /// let unzipped: Result<(Vec<_>, Vec<_>),_> = errorneous.into_iter().try_unzip();
    /// assert_eq!(unzipped, Err("No number found"));
    ///
    ///
    fn try_unzip<A, B, FromA, FromB>(&mut self) -> Result<(FromA, FromB), Self::Err>
    where
        Self: Sized + TryIterator<Ok = (A, B)>,
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
    {
        self.try_fold(
            (FromA::default(), FromB::default()),
            |(mut left_list, mut right_list), couple| {
                let (l, r) = couple?;
                left_list.extend(std::iter::once(l));
                right_list.extend(std::iter::once(r));
                Ok((left_list, right_list))
            },
        )
    }

    /// Fallible version of [`Iterator::max_by_key`]
    /// If every element is a [`Result::Ok`], it has the same behavior.
    ///
    /// - Returns the element that gives the maximum value from the specified function.
    /// - If several elements are equally maximum, the last element is returned.
    /// - If the iterator is empty, [`Option::None`] is returned.
    ///
    /// Otherwise, returns the first error encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use tryiter::TryIteratorExt;
    ///
    /// //       0, 1, 2, 3, 4, 5, 6
    /// let v = [5, 3, 9, 7, 2, 9, 8];
    /// let mut v: Vec<_> = v.into_iter().enumerate().map(Result::Ok).collect();
    /// let max = v.clone().into_iter().try_max_by_key(|(_i, v)| *v);
    /// assert_eq!(max, Ok(Some((5, 9))));
    ///                                                                          
    /// v[3] = Err(7);
    /// v[5] = Err(9);
    /// let max = v.into_iter().try_max_by_key(|(_i, v)| *v);
    /// assert_eq!(max, Err(7));
    /// ```
    fn try_max_by_key<B, F>(mut self, mut f: F) -> Result<Option<Self::Ok>, Self::Err>
    where
        Self: Sized + TryIterator,
        B: Ord,
        F: FnMut(&Self::Ok) -> B,
    {
        match self.next() {
            None => Ok(None),
            Some(Err(e)) => Err(e),
            Some(Ok(v)) => Some(self.try_fold(v, |acc, x| match x {
                Ok(x) => Ok(std::cmp::max_by_key(acc, x, &mut f)),
                Err(e) => Err(e),
            }))
            .transpose(),
        }
    }
}
