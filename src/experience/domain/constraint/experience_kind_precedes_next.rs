use super::Constraint;
use crate::{
    experience::{domain::SelectNextExperience, Error, ExperienceKind, ExperiencedEvent, Result},
    interval::Interval,
};

pub struct ExperienceKindPrecedesNext<'a, Intv> {
    experienced_event: &'a ExperiencedEvent<'a, Intv>,
    next: SelectNextExperience<'a, 'a, Intv>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceKindPrecedesNext<'a, Intv>
where
    Intv: Interval,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<Self> {
        self.next.add(experienced_event);
        Ok(self)
    }

    fn result(self) -> Result<()> {
        let precedes_initial = self
            .next
            .map(|previous| previous.experience)
            .map(ExperienceKind::from)
            .map(|experience| experience.is_initial())
            .unwrap_or_default();

        match self.experienced_event.experience.into() {
            ExperienceKind::Initial => {
                if precedes_initial {
                    return Err(Error::InitialPrecedesInitial);
                }
            }
            ExperienceKind::Transitive => {
                if precedes_initial {
                    return Err(Error::TransitivePrecedesInitial);
                }
            }
            ExperienceKind::Terminal => {
                if !precedes_initial && self.next.is_some() {
                    return Err(Error::TerminalPrecedesNonInitial);
                }
            }
        };

        Ok(())
    }
}

impl<'a, Intv> ExperienceKindPrecedesNext<'a, Intv> {
    pub fn new(experienced_event: &'a ExperiencedEvent<'a, Intv>) -> Self {
        Self {
            experienced_event,
            next: SelectNextExperience::new(experienced_event.event),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        event::{tests::event, Event},
        experience::{
            domain::{constraint::Constraint, ExperienceKindPrecedesNext},
            tests::{initial_experience, terminal_experience, transitive_experience},
            Error, ExperienceBuilder, ExperiencedEvent, Profile, Result,
        },
        id::Id,
        period::Period,
    };
    use std::vec;

    #[test]
    fn experience_kind_precedes_next() {
        struct Test<'a> {
            name: &'a str,
            builder: ExperienceBuilder<'a, Period<usize>>,
            with: Vec<ExperiencedEvent<'a, Period<usize>>>,
            result: Result<()>,
        }

        vec![
            // initial
            Test {
                name: "initial without next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "initial with initial next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &initial_experience(),
                    event: &Event::new(
                        Id::default(),
                        "test".to_string().try_into().unwrap(),
                        [2, 2].into(),
                    ),
                }],
                result: Err(Error::InitialPrecedesInitial),
            },
            Test {
                name: "initial with transitive next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([2, 2]),
                }],
                result: Ok(()),
            },
            Test {
                name: "initial with terminal next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &event([2, 2]),
                }],
                result: Ok(()),
            },
            // transitive
            Test {
                name: "transitive without next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default())))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "transitive with initial next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default())))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &initial_experience(),
                    event: &Event::new(
                        Id::default(),
                        "test".to_string().try_into().unwrap(),
                        [2, 2].into(),
                    ),
                }],
                result: Err(Error::TransitivePrecedesInitial),
            },
            Test {
                name: "transitive with transitive next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default())))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([2, 2]),
                }],
                result: Ok(()),
            },
            Test {
                name: "transitive with terminal next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default())))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &event([2, 2]),
                }],
                result: Ok(()),
            },
            // terminal
            Test {
                name: "terminal without next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "terminal with initial next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![ExperiencedEvent {
                    experience: &initial_experience(),
                    event: &Event::new(
                        Id::default(),
                        "test".to_string().try_into().unwrap(),
                        [2, 2].into(),
                    ),
                }],
                result: Ok(()),
            },
            Test {
                name: "terminal with transitive next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([2, 2]),
                }],
                result: Err(Error::TerminalPrecedesNonInitial),
            },
            Test {
                name: "terminal with terminal next experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &event([2, 2]),
                }],
                result: Err(Error::TerminalPrecedesNonInitial),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let event = test.builder.event;
            let experienced_event = ExperiencedEvent {
                experience: &test.builder.build().unwrap(),
                event,
            };

            let constraint = ExperienceKindPrecedesNext::new(&experienced_event);
            let result = test
                .with
                .iter()
                .try_fold(constraint, |constraint, experienced_event| {
                    constraint.with(experienced_event)
                })
                .and_then(|constraint| constraint.result());

            assert_eq!(
                result, test.result,
                "{} got = {:?}, want {:?}",
                test.name, result, test.result
            );
        });
    }
}
