use plotline::{
    entity::Entity,
    experience::{query::SelectPreviousExperience, Experience},
    id::{Id, Indentify},
    interval::Interval,
    plugin::OutputError,
};
use std::collections::HashSet;

/// An experience cannot belong to an entity not listed in the previous experience.
pub const NOT_IN_PREVIOUS_ERROR: &str = "not_in_previous_experience";

pub struct ExperienceFollowsPrevious<'a, Intv> {
    subject: &'a Experience<Intv>,
    previous: SelectPreviousExperience<'a, 'a, Intv>,
}

impl<'a, Intv> ExperienceFollowsPrevious<'a, Intv>
where
    Intv: Interval,
{
    pub fn with(mut self, experience: &'a Experience<Intv>) -> Self {
        self.previous.add(experience);
        self
    }

    pub fn result(&self) -> std::result::Result<(), OutputError> {
        let Some(previous) = self.previous.as_ref() else {
            return Ok(());
        };

        let previous_profiles = HashSet::<Id<Entity>>::from_iter(
            previous.profiles.iter().map(|profile| profile.entity.id()),
        );

        if previous_profiles.contains(&self.subject.entity.id()) {
            return Ok(());
        }

        Err(
            OutputError::new(NOT_IN_PREVIOUS_ERROR)
                .with_message(
                    format!(
                        "the experience belongs to the entity {} which is not listed in the previous experience {}",
                        self.subject.entity.id(),
                        previous.id()
                    )
                )
        )
    }
}

impl<'a, Intv> ExperienceFollowsPrevious<'a, Intv> {
    pub fn new(subject: &'a Experience<Intv>) -> Self {
        Self {
            subject,
            previous: SelectPreviousExperience::new(&subject.event),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ExperienceFollowsPrevious, NOT_IN_PREVIOUS_ERROR};
    use plotline::{
        entity::Entity,
        experience::{Experience, Profile},
        id::Id,
        moment::Moment,
        period::Period,
        plugin::OutputError,
    };

    #[test]
    fn experience_follows_previous() {
        struct Test<'a> {
            name: &'a str,
            experience: Experience<Period<Moment>>,
            timeline: Vec<Experience<Period<Moment>>>,
            result: std::result::Result<(), OutputError>,
        }

        let const_entity = Entity::default().with_id(Id::default());

        vec![
            Test {
                name: "experience does not belongs to one of previous",
                experience: Experience::fixture([1, 1]),
                timeline: vec![Experience::fixture([0, 0])],
                result: Err(OutputError::new(NOT_IN_PREVIOUS_ERROR)),
            },
            Test {
                name: "experience belongs to one previous",
                experience: Experience::fixture([1, 1]).with_entity(const_entity.clone()),
                timeline: vec![
                    Experience::fixture([0, 0]).with_profiles(vec![Profile::new(const_entity)])
                ],
                result: Ok(()),
            },
            Test {
                name: "experience without previous",
                experience: Experience::fixture([1, 1]),
                timeline: vec![Experience::fixture([2, 2])],
                result: Ok(()),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let constraint = test.timeline.iter().fold(
                ExperienceFollowsPrevious::new(&test.experience),
                |constraint, experience| constraint.with(experience),
            );

            let result = constraint.result();

            assert_eq!(
                result, test.result,
                "{} got = {:?}, want {:?}",
                test.name, result, test.result
            );
        })
    }
}
