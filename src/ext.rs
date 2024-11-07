use std::iter;

use crate::TryIterator;

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
}
