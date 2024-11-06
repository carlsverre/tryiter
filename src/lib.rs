mod ext;
pub use ext::TryIteratorExt;
use private::Sealed;

mod private {
    pub trait Sealed {}
    impl<I, O, E> Sealed for I where I: ?Sized + Iterator<Item = Result<O, E>> {}
}

pub trait TryIterator: Iterator<Item = Result<Self::Ok, Self::Err>> + Sealed {
    /// The type of successful values yielded by this iterator
    type Ok;

    /// The type of failures yielded by this iterator
    type Err;
}

impl<I, O, E> TryIterator for I
where
    I: ?Sized + Iterator<Item = Result<O, E>>,
{
    type Ok = O;
    type Err = E;
}

impl<I: TryIterator> TryIteratorExt for I {}
