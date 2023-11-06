/// A Guard holds an instance of T among other data to ensure its atomicity in front of
/// mutations.
pub trait Guard<T>: AsRef<T> + AsMut<T> {}
