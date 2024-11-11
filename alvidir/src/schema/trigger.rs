pub struct WithTrigger<T> {
    pub inner: T,
}

impl<T> From<T> for WithTrigger<T> {
    fn from(inner: T) -> Self {
        WithTrigger { inner }
    }
}
