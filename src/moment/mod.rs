mod iterator;
pub use iterator::*;

use std::sync::Arc;

/// A Moment answers the "when", giving the order of time.
#[derive(Debug, Default)]
pub struct Moment {
    /// The previous moment immediately before self.
    before: Option<Arc<Moment>>,
    /// The next moment immediately after self.
    after: Option<Arc<Moment>>,
}

impl PartialEq for Moment {
    /// A Moment equals no other moment but itself, even if both share the exact same values.
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl PartialOrd for Moment {
    /// A Moment is comparable to another as long as both belong to the same timeline.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }

        // let self_timeline: Timeline = self.before.clone().unwrap_or_default().into();
        // let other_timeline: Timeline = other.before.clone().unwrap_or_default().into();

        // for (self_before, other_before) in self_timeline.rev().zip(other_timeline.rev()) {}

        None
    }
}

impl Moment {
    pub fn with_before(mut self, before: Arc<Moment>) -> Self {
        self.before = Some(before);
        self
    }

    pub fn with_after(mut self, after: Arc<Moment>) -> Self {
        self.after = Some(after);
        self
    }
}

/// A Period represents the time being between two different [Moment]s in time.
pub struct Period([Arc<Moment>; 2]);

/// A Duration represents the time during which something takes place.
pub enum Duration {
    Moment(Arc<Moment>),
    Period(Arc<Period>),
}

#[cfg(test)]
mod tests {
    use crate::moment::Moment;

    #[test]
    fn moment_must_equals_to_itself_only() {
        let moment_a = Moment::default();
        assert_eq!(moment_a, moment_a, "a moment must equals itself");

        let moment_b = Moment::default();
        assert_ne!(moment_a, moment_b, "two equivalent moments must not equal");
    }

    #[test]
    fn moment_must_compare_in_its_timeline_only() {
        let moment_a = Moment::default();
        let moment_b = Moment::default();
    }
}
