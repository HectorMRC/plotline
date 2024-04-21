use crate::error::Error;
use plotline::{experience::Experience, id::Indentify, interval::Interval};

pub struct ExperienceIsNotSimultaneous<'a, Intv> {
    subject: &'a Experience<Intv>,
    conflict: Option<Experience<Intv>>,
}

impl<'a, Intv> ExperienceIsNotSimultaneous<'a, Intv>
where
    Intv: Interval,
{
    pub fn with(mut self, experience: &Experience<Intv>) -> Self {
        if self.subject.event.intersects(&experience.event) {
            self.conflict = Some(experience.clone());
        }

        self
    }

    pub fn result(&self) -> std::result::Result<(), Error> {
        if let Some(conflict) = &self.conflict {
            return Err(
                 Error::default()
                    .with_details(
                        format!(
                            "The entity {} would be experiencing the event {} and the event {} (because of experience {}) simultaneously, which should be impossible.",
                            self.subject.entity.id(),
                            self.subject.event.id(),
                            conflict.event.id(),
                            conflict.id()
                        )
                    )
            );
        }

        Ok(())
    }
}

impl<'a, Intv> ExperienceIsNotSimultaneous<'a, Intv> {
    pub fn new(subject: &'a Experience<Intv>) -> Self {
        Self {
            subject,
            conflict: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExperienceIsNotSimultaneous;
    use crate::error::Error;
    use plotline::{
        experience::{Experience, Profile},
        moment::Moment,
        period::Period,
    };

    impl PartialEq for Error {
        fn eq(&self, other: &Self) -> bool {
            self.error == other.error
        }
    }

    #[test]
    fn experience_is_not_simultaneous() {
        struct Test<'a> {
            name: &'a str,
            experience: Experience<Period<Moment>>,
            with: Vec<Experience<Period<Moment>>>,
            result: std::result::Result<(), Error>,
        }

        vec![
            Test {
                name: "experience without surroundings",
                experience: Experience::fixture([1, 3]),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "experience with previous",
                experience: Experience::fixture([1, 3]),
                with: vec![Experience::fixture([0, 0]).with_profiles(vec![Profile::fixture()])],
                result: Ok(()),
            },
            Test {
                name: "experience with previous overlapping",
                experience: Experience::fixture([1, 3]),
                with: vec![Experience::fixture([0, 1]).with_profiles(vec![Profile::fixture()])],
                result: Err(Error::default()),
            },
            Test {
                name: "experience with partial overlapping",
                experience: Experience::fixture([1, 3]),
                with: vec![Experience::fixture([2, 2]).with_profiles(vec![Profile::fixture()])],
                result: Err(Error::default()),
            },
            Test {
                name: "experience with total overlapping",
                experience: Experience::fixture([1, 3]),
                with: vec![Experience::fixture([1, 3]).with_profiles(vec![Profile::fixture()])],
                result: Err(Error::default()),
            },
            Test {
                name: "experience with next overlapping",
                experience: Experience::fixture([1, 3]),
                with: vec![Experience::fixture([3, 4]).with_profiles(vec![Profile::fixture()])],
                result: Err(Error::default()),
            },
            Test {
                name: "experience with next",
                experience: Experience::fixture([1, 3]),
                with: vec![Experience::fixture([4, 4]).with_profiles(vec![Profile::fixture()])],
                result: Ok(()),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let constraint = test.with.into_iter().fold(
                ExperienceIsNotSimultaneous::new(&test.experience),
                |constraint, experience| constraint.with(&experience),
            );

            let result = constraint.result();

            assert_eq!(
                result, test.result,
                "{} got = {:?}, want = {:?}",
                test.name, result, test.result
            );
        });
    }
}