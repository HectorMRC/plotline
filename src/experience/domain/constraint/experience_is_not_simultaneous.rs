use super::Constraint;
use crate::{
    experience::{Error, ExperienceBuilder, ExperiencedEvent, Result},
    interval::Interval,
};

pub struct ExperienceIsNotSimultaneous<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    conflict: Option<&'a ExperiencedEvent<'a, Intv>>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceIsNotSimultaneous<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<()> {
        if self.builder.event.intersects(experienced_event.event) {
            self.conflict = Some(experienced_event);
        }

        self.result()
    }

    fn result(&self) -> Result<()> {
        if self.conflict.is_some() {
            return Err(Error::SimultaneousEvents);
        }

        Ok(())
    }
}

impl<'a, Intv> ExperienceIsNotSimultaneous<'a, Intv> {
    pub fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self {
            builder,
            conflict: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        event::tests::event,
        experience::{
            domain::{Constraint, ExperienceIsNotSimultaneous},
            tests::transitive_experience,
            Error, ExperienceBuilder, ExperiencedEvent, Result,
        },
        period::Period,
    };

    #[test]
    fn experience_is_not_simultaneous() {
        struct Test<'a> {
            name: &'a str,
            builder: ExperienceBuilder<'a, Period<usize>>,
            with: Vec<ExperiencedEvent<'a, Period<usize>>>,
            result: Result<()>,
        }

        vec![
            Test {
                name: "experience without surroundings",
                builder: ExperienceBuilder::new(&event([1, 3])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "experience with previous",
                builder: ExperienceBuilder::new(&event([1, 3])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([0, 0]),
                }],
                result: Ok(()),
            },
            Test {
                name: "experience with previous overlapping",
                builder: ExperienceBuilder::new(&event([1, 3])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([0, 1]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with partial overlapping",
                builder: ExperienceBuilder::new(&event([1, 3])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([2, 2]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with total overlapping",
                builder: ExperienceBuilder::new(&event([1, 3])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([1, 3]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with next overlapping",
                builder: ExperienceBuilder::new(&event([1, 3])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([3, 4]),
                }],
                result: Err(Error::SimultaneousEvents),
            },
            Test {
                name: "experience with next",
                builder: ExperienceBuilder::new(&event([1, 3])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([4, 4]),
                }],
                result: Ok(()),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let mut constraint = ExperienceIsNotSimultaneous::new(&test.builder);
            let result = test
                .with
                .iter()
                .try_for_each(|experienced_event| constraint.with(experienced_event));

            if result.is_err() {
                assert_eq!(
                    result, test.result,
                    "{} got = {:?}, want = {:?}",
                    test.name, result, test.result
                );
            }

            let result = constraint.result();
            assert_eq!(
                result, test.result,
                "{} got = {:?}, want = {:?}",
                test.name, result, test.result
            );
        });
    }
}
