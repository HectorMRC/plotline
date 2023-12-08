use super::Constraint;
use crate::{
    event::Event,
    experience::{Error, ExperiencedEvent, Result},
    interval::Interval,
};

pub struct ExperienceIsNotSimultaneous<'a, Intv> {
    event: &'a Event<Intv>,
    conflict: Option<&'a ExperiencedEvent<'a, Intv>>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceIsNotSimultaneous<'a, Intv>
where
    Intv: Interval,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<Self> {
        if self.event.intersects(experienced_event.event) {
            self.conflict = Some(experienced_event);
        }

        Ok(self)
    }

    fn result(self) -> Result<()> {
        if self.conflict.is_some() {
            return Err(Error::SimultaneousEvents);
        }

        Ok(())
    }
}

impl<'a, Intv> ExperienceIsNotSimultaneous<'a, Intv> {
    pub fn new(event: &'a Event<Intv>) -> Self {
        Self {
            event,
            conflict: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        event::{tests::event, Event},
        experience::{
            domain::{Constraint, ExperienceIsNotSimultaneous},
            tests::transitive_experience,
            Error, ExperiencedEvent, Result,
        },
        period::Period,
    };

    #[test]
    fn experience_is_not_simultaneous() {
        struct Test<'a> {
            name: &'a str,
            event: Event<Period<usize>>,
            with: Vec<ExperiencedEvent<'a, Period<usize>>>,
            result: Result<()>,
        }

        vec![
            Test {
                name: "experience without surroundings",
                event: event([1, 3]),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "experience with previous",
                event: event([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([0, 0]),
                }],
                result: Ok(()),
            },
            Test {
                name: "experience with previous overlapping",
                event: event([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([0, 1]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with partial overlapping",
                event: event([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([2, 2]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with total overlapping",
                event: event([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([1, 3]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with next overlapping",
                event: event([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([3, 4]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with next",
                event: event([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([4, 4]),
                }],
                result: Ok(()),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let constraint = ExperienceIsNotSimultaneous::new(&test.event);
            let result = test
                .with
                .iter()
                .try_fold(constraint, |constraint, experienced_event| {
                    constraint.with(experienced_event)
                })
                .and_then(|constraint| constraint.result());

            assert_eq!(
                result, test.result,
                "{} got = {:?}, want = {:?}",
                test.name, result, test.result
            );
        });
    }
}
