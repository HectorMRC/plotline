use std::sync::Arc;

/// A Timeline represents an ordered chain of [Moment]s.
#[derive(Default)]
pub struct Timeline {
    moments: Vec<Moment>,
}

impl PartialEq for Timeline {
    /// A Timeline equals no other timeline but itself.
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Timeline {
    pub fn moments(&self) -> &[Moment] {
        &self.moments
    }

    pub fn push_moment(&mut self, moment: Moment) {
        self.moments.push(moment);
    }
}

/// A Moment answers the "when", giving the order of time.
pub struct Moment {
    timeline: Box<Timeline>,
}

impl PartialEq for Moment {
    /// A Moment equals no other moment but itself.
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl PartialOrd for Moment {
    /// A Moment is comparable with another as long as both belong to the same timeline.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }

        if self.timeline != other.timeline {
            return None;
        }

        for moment in self.timeline.moments.iter() {
            if other == moment {
                return Some(std::cmp::Ordering::Greater);
            }

            if self == moment {
                return Some(std::cmp::Ordering::Less);
            }
        }

        None
    }
}

// impl Moment {
//     fn new(timeline: &mut Timeline) -> Self {
//         Self {
//             timeline: Box::new(timeline),
//         }
//     }
// }

// /// A Period represents the time being between two different [Moment]s in time.
// pub struct Period([&'a Moment; 2]);

// /// A Duration represents the time during which something takes place.
// pub enum Duration {
//     Moment(&'a Moment),
//     Period(&'a Period),
// }

#[cfg(test)]
mod tests {
    use std::{ops::Deref, sync::Arc};

    use crate::timeline::{self, Moment, Timeline};

    // #[test]
    // fn moment_must_equals_to_itself_only() {
    //     let mut timeline = Timeline::default();

    //     let moment_a = Moment::new(&timeline);
    //     timeline.add_moment(moment_a);

    //     assert!(moment_a == moment_a, "a moment must equals itself");

    //     // let moment_b = Moment::default();
    //     // assert_ne!(moment_a, moment_b, "two equivalent moments must not equal");
    // }

    // #[test]
    // fn moment_must_compare_in_its_timeline_only() {
    //     let moment_a = Moment::default();
    //     let moment_b = Moment::default();

    //     assert!(!(moment_a == moment_b));
    //     assert!(!(moment_a > moment_b));
    //     assert!(!(moment_a < moment_b));
    //     assert!(moment_a != moment_b);

    //     let moment_a = Arc::new(moment_a);
    //     let moment_b = Arc::new(moment_b);
    //     let moment_c = Moment::default()
    //         .with_after(moment_a.clone())
    //         .with_before(moment_b.clone());

    //     assert!(
    //         &moment_c > moment_b.deref(),
    //         "a moment after must be greater than a moment before"
    //     );

    //     assert!(
    //         &moment_c < moment_a.deref(),
    //         "a moment before must be lower than a moment after"
    //     );

    //     assert!(
    //         moment_a.deref() > &moment_c,
    //         "a moment after must be greater than a moment before"
    //     );

    //     // assert!(
    //     //     moment_b.deref() < &moment_c,
    //     //     "a moment before must be lower than a moment after"
    //     // );

    //     // assert!(
    //     //     moment_a > moment_b,
    //     //     "a moment after must be greater than a moment before"
    //     // );

    //     // assert!(
    //     //     moment_b < moment_a,
    //     //     "a moment before must be lower than a moment after"
    //     // );
    // }
}
