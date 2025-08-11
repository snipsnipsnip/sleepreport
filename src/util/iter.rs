pub trait IterExt: Sized + IntoIterator {
    fn into_vec(self) -> Vec<Self::Item> {
        Vec::from_iter(self)
    }

    fn into_try_vec<T, E>(self) -> Result<Vec<T>, E>
    where
        Result<Vec<T>, E>: FromIterator<Self::Item>,
    {
        FromIterator::from_iter(self)
    }
}

impl<T: IntoIterator> IterExt for T {}
