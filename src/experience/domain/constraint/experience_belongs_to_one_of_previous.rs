use super::Constraint;
use crate::{
    entity::Entity,
    experience::{
        domain::SelectPreviousExperience, Error, ExperienceBuilder, ExperiencedEvent, Result,
    },
    id::Id,
    interval::Interval,
};
use std::collections::HashSet;

pub struct ExperienceBelongsToOneOfPrevious<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    previous: SelectPreviousExperience<'a, 'a, Intv>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceBelongsToOneOfPrevious<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<()> {
        self.previous.add(experienced_event);
        Ok(())
    }

    fn result(&self) -> Result<()> {
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

        if self
            .builder
            .before
            .as_ref()
            .map(|before| previous_afters.contains(&before.entity))
            .unwrap_or_default()
        {
            return Ok(());
        }

        Err(Error::NotInPreviousExperience)
    }
}

impl<'a, Intv> ExperienceBelongsToOneOfPrevious<'a, Intv> {
    pub fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self {
            builder,
            previous: SelectPreviousExperience::from_builder(builder),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        event::{tests::event, Event},
        experience::{
            domain::{Constraint, ExperienceBelongsToOneOfPrevious},
            tests::{initial_experience, terminal_experience, transitive_experience},
            Error, ExperienceBuilder, ExperiencedEvent, Profile, Result,
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
            // initial
            Test {
                name: "initial without previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "initial with non-terminal previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &initial_experience(),
                    event: &Event::new(
                        Id::default(),
                        "test".to_string().try_into().unwrap(),
                        [0, 0].into(),
                    ),
                }],
                result: Err(Error::NotInPreviousExperience),
            },
            Test {
                name: "initial with terminal previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &event([0, 0]),
                }],
                result: Ok(()),
            },
            // transitive
            Test {
                name: "transitive without previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default())))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "transitive belongs to non-terminal previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(const_id)))
                    .with_after(Some(vec![Profile::new(const_id)])),
                with: vec![ExperiencedEvent {
                    experience: &{
                        let mut initial = initial_experience();
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
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default())))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &initial_experience(),
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
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default())))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &event([0, 0]),
                }],
                result: Ok(()),
            },
            // // terminal
            Test {
                name: "terminal without previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "terminal belongs to non-terminal previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(const_id))),
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
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(const_id))),
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
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &event([0, 0]),
                }],
                result: Ok(()),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let mut constraint = ExperienceBelongsToOneOfPrevious::new(&test.builder);
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
                "{} got = {:?}, want {:?}",
                test.name, result, test.result
            );
        })
    }
}
