use super::{Constraint, Error, Recoverable, Result};
use crate::{error::PoisonError, event::Event, experience::ExperiencedEvent, interval::Interval};

pub struct EventIsNotExperiencedMoreThanOnce<'a, Intv> {
    event: &'a Event<Intv>,
    already_experienced: bool,
}

impl<'a, Intv> Constraint<'a, Intv> for EventIsNotExperiencedMoreThanOnce<'a, Intv>
where
    Intv: Interval,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Recoverable<Self> {
        self.already_experienced = self.event == experienced_event.event;
        if self.already_experienced {
            return Err(PoisonError::new(self, Error::EventAlreadyExperienced));
        }

        Ok(self)
    }

    fn result(self) -> Result<()> {
        if self.already_experienced {
            return Err(Error::EventAlreadyExperienced);
        }

        Ok(())
    }
}

impl<'a, Intv> EventIsNotExperiencedMoreThanOnce<'a, Intv> {
    pub fn new(event: &'a Event<Intv>) -> Self {
        Self {
            event,
            already_experienced: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        event::Event,
        experience::{
            constraint::{Constraint, Error, EventIsNotExperiencedMoreThanOnce, Result},
            tests::transitive_experience,
            ExperiencedEvent,
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
                event: Event::fixture([1, 3]),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "experience with previous",
                event: Event::fixture([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::fixture([0, 0]),
                }],
                result: Ok(()),
            },
            Test {
                name: "experience with previous overlapping",
                event: Event::fixture([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::fixture([0, 1]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with partial overlapping",
                event: Event::fixture([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::fixture([2, 2]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with total overlapping",
                event: Event::fixture([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::fixture([1, 3]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with next overlapping",
                event: Event::fixture([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::fixture([3, 4]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with next",
                event: Event::fixture([1, 3]),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::fixture([4, 4]),
                }],
                result: Ok(()),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let constraint = EventIsNotExperiencedMoreThanOnce::new(&test.event);
            let result = test
                .with
                .iter()
                .try_fold(constraint, |constraint, experienced_event| {
                    constraint.with(experienced_event)
                })
                .map_err(Into::into)
                .and_then(|constraint| constraint.result());

            assert_eq!(
                result, test.result,
                "{} got = {:?}, want = {:?}",
                test.name, result, test.result
            );
        });
    }
}
