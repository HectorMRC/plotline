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

pub struct ExperienceMustBelongToOneOfPrevious<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    previous: SelectPreviousExperience<'a, 'a, Intv>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceMustBelongToOneOfPrevious<'a, Intv>
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

impl<'a, Intv> ExperienceMustBelongToOneOfPrevious<'a, Intv> {
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
        event::{Event, tests::event},
        experience::{
            domain::{Constraint, ExperienceMustBelongToOneOfPrevious},
            ExperienceBuilder, ExperiencedEvent, Profile, Result, tests::initial_experience, Error,
        },
        id::Id,
        period::Period,
    };

    #[test]
    fn experience_must_belong_to_one_of_previous() {
        struct Test<'a> {
            name: &'a str,
            builder: ExperienceBuilder<'a, Period<usize>>,
            with: Vec<ExperiencedEvent<'a, Period<usize>>>,
            result: Result<()>,
        }

        vec![
            Test {
                name: "initial without previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "intial with initial previous experience",
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
                result: Err(Error::NotInPreviousExperience)
            },
        ]
        .into_iter()
        .for_each(|test| {
            let mut constraint = ExperienceMustBelongToOneOfPrevious::new(&test.builder);
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
