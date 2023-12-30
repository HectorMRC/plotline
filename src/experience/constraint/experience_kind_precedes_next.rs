use super::{Constraint, Result, Error};
use crate::{
    experience::{query::SelectNextExperience, ExperienceKind, ExperiencedEvent},
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
        let precedes_terminal = self
            .next
            .map(|previous| previous.experience)
            .map(ExperienceKind::from)
            .map(|experience| experience.is_terminal())
            .unwrap_or_default();

        match self.experienced_event.experience.into() {
            ExperienceKind::Terminal if precedes_terminal => Err(Error::TerminalPrecedesTerminal),
            _ => Ok(()),
        }
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
        entity::Entity,
        event::Event,
        experience::{
            constraint::{Constraint, ExperienceKindPrecedesNext, Result, Error},
            tests::{terminal_experience, transitive_experience},
            ExperienceBuilder, ExperiencedEvent, Profile,
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
            // transitive
            Test {
                name: "transitive without next experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "transitive with transitive next experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::fixture([2, 2]),
                }],
                result: Ok(()),
            },
            Test {
                name: "transitive with terminal next experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1]))
                    .with_after(Some(vec![Profile::new(Id::default())])),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &Event::fixture([2, 2]),
                }],
                result: Ok(()),
            },
            // terminal
            Test {
                name: "terminal without next experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1])),
                with: vec![],
                result: Ok(()),
            },
            Test {
                name: "terminal with transitive next experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1])),
                with: vec![ExperiencedEvent {
                    experience: &transitive_experience(),
                    event: &Event::fixture([2, 2]),
                }],
                result: Ok(()),
            },
            Test {
                name: "terminal with terminal next experience",
                builder: ExperienceBuilder::new(&Entity::fixture(), &Event::fixture([1, 1])),
                with: vec![ExperiencedEvent {
                    experience: &terminal_experience(),
                    event: &Event::fixture([2, 2]),
                }],
                result: Err(Error::TerminalPrecedesTerminal),
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
