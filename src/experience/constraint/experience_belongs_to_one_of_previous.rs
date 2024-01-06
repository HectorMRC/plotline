use super::{Constraint, ConstraintResult, Error, Result};
use crate::{
    entity::Entity,
    experience::{query::SelectPreviousExperience, ExperiencedEvent},
    id::Id,
    interval::Interval,
};
use std::collections::HashSet;

pub struct ExperienceBelongsToOneOfPrevious<'a, Intv> {
    experienced_event: &'a ExperiencedEvent<'a, Intv>,
    previous: SelectPreviousExperience<'a, 'a, Intv>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceBelongsToOneOfPrevious<'a, Intv>
where
    Intv: Interval,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> ConstraintResult<Self> {
        self.previous.add(experienced_event);
        Ok(self)
    }

    fn result(self) -> Result<()> {
        let Some(previous) = self.previous.as_ref() else {
            return Ok(());
        };

        let previous_afters = HashSet::<Id<Entity>>::from_iter(
            previous
                .experience
                .after
                .iter()
                .map(|profile| profile.entity),
        );

        if previous_afters.is_empty() {
            return Ok(());
        }

        if previous_afters.contains(&self.experienced_event.experience.entity) {
            return Ok(());
        }

        Err(Error::NotInPreviousExperience)
    }
}

impl<'a, Intv> ExperienceBelongsToOneOfPrevious<'a, Intv> {
    pub fn new(experienced_event: &'a ExperiencedEvent<'a, Intv>) -> Self {
        Self {
            experienced_event,
            previous: SelectPreviousExperience::new(experienced_event.event),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entity::Entity,
        event::Event,
        experience::{
            constraint::{Constraint, Error, ExperienceBelongsToOneOfPrevious, Result},
            tests::{terminal_experience, transitive_experience},
            ExperienceBuilder, ExperiencedEvent, Profile,
        },
        id::Id,
        period::Period,
    };

    #[test]
    fn experience_belongs_to_one_of_previous() {
        struct Test<'a> {
            name: &'a str,
            builder: ExperienceBuilder<'a, Period<usize>>,
            with: Vec<ExperiencedEvent<'a, Period<usize>>>,
            result: Result<()>,
        }

        let const_id = Id::default();

        vec![
            // transitive
            Test {
                name: "transitive without previous experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "transitive belongs to non-terminal previous experience",
                builder: ExperienceBuilder::new(
                    &Entity::fixture().with_id(const_id),
                    &Event::fixture([1, 1]),
                )
                .with_after(Some(vec![Profile::new(const_id)])),
                with: vec![ExperiencedEvent {
                    experience: &{
                        let mut initial = transitive_experience();
                        initial
                            .after
                            .iter_mut()
                            .for_each(|profile| profile.entity = const_id);
                        initial
                    },
                    event: &Event::new(
                        Id::default(),
                        "test".to_string().try_into().unwrap(),
                        [0, 0].into(),
                    ),
                }],
                result: Ok(()),
            },
            Test {
                name: "transitive does not belong to non-terminal previous experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::new(
                        Id::default(),
                        "test".to_string().try_into().unwrap(),
                        [0, 0].into(),
                    ),
                }],
                result: Err(Error::NotInPreviousExperience),
            },
            Test {
                name: "transitive with terminal previous experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &Event::fixture([0, 0]),
                }],
                result: Ok(()),
            },
            // terminal
            Test {
                name: "terminal without previous experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "terminal belongs to non-terminal previous experience",
                builder: ExperienceBuilder::new(
                    &Entity::fixture().with_id(const_id),
                    &Event::fixture([1, 1]),
                ),
                with: vec![ExperiencedEvent {
                    experience: &{
                        let mut initial = transitive_experience();
                        initial
                            .after
                            .iter_mut()
                            .for_each(|profile| profile.entity = const_id);
                        initial
                    },
                    event: &Event::new(
                        Id::default(),
                        "test".to_string().try_into().unwrap(),
                        [0, 0].into(),
                    ),
                }],
                result: Ok(()),
            },
            Test {
                name: "terminal does not belong to non-terminal previous experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::new(
                        Id::default(),
                        "test".to_string().try_into().unwrap(),
                        [0, 0].into(),
                    ),
                }],
                result: Err(Error::NotInPreviousExperience),
            },
            Test {
                name: "terminal with terminal previous experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1])),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &Event::fixture([0, 0]),
                }],
                result: Ok(()),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let event = test.builder.event;
            let experienced_event = ExperiencedEvent {
                experience: &test.builder.build().unwrap(),
                event,
            };

            let constraint = ExperienceBelongsToOneOfPrevious::new(&experienced_event);
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
                "{} got = {:?}, want {:?}",
                test.name, result, test.result
            );
        })
    }
}
