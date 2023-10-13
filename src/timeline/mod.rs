pub mod error;

use std::sync::Arc;

/// A Timeline represents an ordered chain of [Moment]s.
#[derive(Default)]
pub struct Timeline {
    moments: Vec<Arc<Moment>>,
}

impl PartialEq for Timeline {
    /// A Timeline equals no other timeline but itself.
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Timeline {
    pub fn moments(&self) -> &[Arc<Moment>] {
        &self.moments
    }
}

/// A Moment answers the "when", giving the order of time.
#[derive(Default)]
pub struct Moment {
    
}

impl PartialEq for Moment {
    /// A Moment equals no other moment but itself.
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

/// A Period represents the time being between two different [Moment]s in time, both included.
pub struct Period([Arc<Moment>; 2]);

impl Period {
    pub fn new(start: Arc<Moment>, end: Arc<Moment>) -> Self {
        Self([start, end])
    }
}

/// A Duration represents the time during which something takes place.
pub enum Duration {
    Moment(Arc<Moment>),
    Period(Arc<Period>),
}

#[cfg(test)]
mod tests {
    use super::Moment;

    #[test]
    fn a_timeline_must_not_equal_other_but_itself() {
        let timeline_a = Moment::default();
        assert!(timeline_a == timeline_a, "a timeline must equals itself");

        let timeline_b = Moment::default();
        assert!(
            timeline_a != timeline_b,
            "two equivalent timelines must not equal"
        );
    }

    #[test]
    fn a_moment_must_not_equal_other_but_itself() {
        let moment_a = Moment::default();
        assert!(moment_a == moment_a, "a moment must equals itself");

        let moment_b = Moment::default();
        assert!(
            moment_a != moment_b,
            "two equivalent moments must not equal"
        );
    }
}
