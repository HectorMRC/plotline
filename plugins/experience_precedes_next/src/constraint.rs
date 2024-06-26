use plotline::{
    experience::{query::SelectNextExperience, Experience},
    interval::Interval,
    plugin::{self, OutputError},
};

/// An experience cannot precede another experience which entity is not listed in the current one.
pub const NOT_IN_NEXT_ERROR: &str = "not_in_next_experience";

pub struct ExperiencePrecedesNext<'a, Intv> {
    subject: &'a Experience<Intv>,
    next: SelectNextExperience<'a, 'a, Intv>,
}

impl<'a, Intv> ExperiencePrecedesNext<'a, Intv>
where
    Intv: Interval,
{
    pub fn with(mut self, experience: &'a Experience<Intv>) -> Self {
        self.next.add(experience);
        self
    }

    pub fn result(&self) -> std::result::Result<(), OutputError> {
        let Some(next) = self.next.as_ref() else {
            return Ok(());
        };

        if self
            .subject
            .profiles
            .iter()
            .any(|profile| profile.entity == next.entity)
        {
            return Ok(());
        }

        Err(plugin::OutputError::new(NOT_IN_NEXT_ERROR).with_message(
            "the experience belongs an entity which is not listed in the next experience",
        ))
    }
}

impl<'a, Intv> ExperiencePrecedesNext<'a, Intv> {
    pub fn new(subject: &'a Experience<Intv>) -> Self {
        Self {
            subject,
            next: SelectNextExperience::new(&subject.event),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ExperiencePrecedesNext, NOT_IN_NEXT_ERROR};
    use plotline::{
        entity::Entity,
        experience::{Experience, Profile},
        id::Id,
        moment::Moment,
        period::Period,
        plugin::OutputError,
    };

    #[test]
    fn experience_precedes_next() {
        struct Test<'a> {
            name: &'a str,
            experience: Experience<Period<Moment>>,
            timeline: Vec<Experience<Period<Moment>>>,
            result: std::result::Result<(), OutputError>,
        }

        let const_entity = Entity::default().with_id(Id::default());

        vec![
            Test {
                name: "experience does not precedes next",
                experience: Experience::fixture([1, 1]),
                timeline: vec![Experience::fixture([2, 2])],
                result: Err(OutputError::new(NOT_IN_NEXT_ERROR)),
            },
            Test {
                name: "experience precedes next",
                experience: Experience::fixture([1, 1])
                    .with_profiles(vec![Profile::new(const_entity.clone())]),
                timeline: vec![Experience::fixture([2, 2]).with_entity(const_entity.clone())],
                result: Ok(()),
            },
            Test {
                name: "experience without next",
                experience: Experience::fixture([1, 1]),
                timeline: vec![Experience::fixture([0, 0])],
                result: Ok(()),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let constraint = test.timeline.iter().fold(
                ExperiencePrecedesNext::new(&test.experience),
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
