use super::Constraint;
use crate::{
    experience::{
        domain::SelectPreviousExperience, Error, ExperienceKind, ExperiencedEvent, Result,
    },
    interval::Interval,
};

pub struct ExperienceKindFollowsPrevious<'a, Intv> {
    experienced_event: &'a ExperiencedEvent<'a, Intv>,
    previous: SelectPreviousExperience<'a, 'a, Intv>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceKindFollowsPrevious<'a, Intv>
where
    Intv: Interval,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<Self> {
        self.previous.add(experienced_event);
        Ok(self)
    }

    fn result(self) -> Result<()> {
        let follows_terminal = self
            .previous
            .map(|previous| previous.experience)
            .map(ExperienceKind::from)
            .map(|experience| experience.is_terminal())
            .unwrap_or(true);

        match self.experienced_event.experience.into() {
            ExperienceKind::Initial => {
                if !follows_terminal {
                    return Err(Error::InitialFollowsNonTerminal);
                }
            }
            ExperienceKind::Transitive => {
                if follows_terminal {
                    return Err(Error::TransitiveFollowsTerminal);
                }
            }
            ExperienceKind::Terminal => {
                if follows_terminal {
                    return Err(Error::TerminalFollowsTerminal);
                }
            }
        };

        Ok(())
    }
}

impl<'a, Intv> ExperienceKindFollowsPrevious<'a, Intv> {
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
        event::{tests::event, Event},
        experience::{
            domain::{constraint::Constraint, ExperienceKindFollowsPrevious},
            tests::{initial_experience, terminal_experience, transitive_experience},
            Error, ExperienceBuilder, ExperiencedEvent, Profile, Result,
        },
        id::Id,
        period::Period,
    };
    use std::vec;

    #[test]
    fn experience_kind_follows_previous() {
        struct Test<'a> {
            name: &'a str,
            builder: ExperienceBuilder<'a, Period<usize>>,
            with: Vec<ExperiencedEvent<'a, Period<usize>>>,
            result: Result<()>,
        }

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
                name: "initial with initial previous experience",
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
                result: Err(Error::InitialFollowsNonTerminal),
            },
            Test {
                name: "initial with transitive previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([0, 0]),
                }],
                result: Err(Error::InitialFollowsNonTerminal),
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
                result: Err(Error::TransitiveFollowsTerminal),
            },
            Test {
                name: "transitive with initial previous experience",
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
                result: Ok(()),
            },
            Test {
                name: "transitive with transitive previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default())))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([0, 0]),
                }],
                result: Ok(()),
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
                result: Err(Error::TransitiveFollowsTerminal),
            },
            // terminal
            Test {
                name: "terminal without previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![],
                result: Err(Error::TerminalFollowsTerminal),
            },
            Test {
                name: "terminal with initial previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![ExperiencedEvent {
                    experience: &initial_experience(),
                    event: &Event::new(
                        Id::default(),
                        "test".to_string().try_into().unwrap(),
                        [0, 0].into(),
                    ),
                }],
                result: Ok(()),
            },
            Test {
                name: "terminal with transitive previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &event([0, 0]),
                }],
                result: Ok(()),
            },
            Test {
                name: "terminal with terminal previous experience",
                builder: ExperienceBuilder::new(&event([1, 1]))
                    .with_before(Some(Profile::new(Id::default()))),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &event([0, 0]),
                }],
                result: Err(Error::TerminalFollowsTerminal),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let event = test.builder.event;
            let experienced_event = ExperiencedEvent {
                experience: &test.builder.build().unwrap(),
                event,
            };

            let constraint = ExperienceKindFollowsPrevious::new(&experienced_event);
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
