pub trait IterExt: Sized + IntoIterator {
    fn into_vec(self) -> Vec<Self::Item> {
        Vec::from_iter(self)
    }
}

impl<T: IntoIterator> IterExt for T {}
