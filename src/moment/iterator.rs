use super::Moment;
use std::sync::Arc;

/// A Timeline implements the double ended iterator for a chain of [Moment]s.
pub struct Timeline {
    next: Option<Arc<Moment>>,
}

impl From<Arc<Moment>> for Timeline {
    fn from(value: Arc<Moment>) -> Self {
        Timeline { next: Some(value) }
    }
}

impl Iterator for Timeline {
    type Item = Arc<Moment>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.take();
        if let Some(current) = current.as_deref() {
            self.next = current.after.clone();
        };

        current
    }
}

impl DoubleEndedIterator for Timeline {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current = self.next.take();
        if let Some(current) = current.as_deref() {
            self.next = current.before.clone();
        };

        current
    }
}
