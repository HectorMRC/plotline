use super::Constraint;
use crate::{
    experience::{Error, ExperienceBuilder, ExperienceKind, ExperiencedEvent, Result},
    interval::Interval,
};

/// InitialMustBeBeforeOrEqual makes sure no experience takes places before the
/// initial experience of the corresponding [Entity].
pub struct InitialMustBeBeforeOrEqual<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    initial: Option<&'a ExperiencedEvent<'a, Intv>>,
}

impl<'a, Intv> Constraint<'a, Intv> for InitialMustBeBeforeOrEqual<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<()> {
        let kind: ExperienceKind = experienced_event.experience.into();
        if kind.is_initial() {
            self.initial = Some(experienced_event);
        }

        self.result()
    }

    fn result(&self) -> Result<()> {
        let Some(initial) = self.initial else {
            return Ok(());
        };

        if self.builder.event < initial.event {
            return Err(Error::ExperienceBeforeInitial);
        }

        Ok(())
    }
}

impl<'a, Intv> InitialMustBeBeforeOrEqual<'a, Intv> {
    pub fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self {
            builder,
            initial: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        event::Event,
        experience::{
            domain::{constraint::Constraint, InitialMustBeBeforeOrEqual},
            ExperienceBuilder, ExperiencedEvent, Result,
        },
        id::Id,
        period::Period,
    };

    #[test]
    fn initial_must_be_before_or_equal() {
        struct Test<'a> {
            name: &'a str,
            builder: ExperienceBuilder<'a, Period<usize>>,
            with: Vec<ExperiencedEvent<'a, Period<usize>>>,
            result: Result<()>,
        }

        vec![Test {
            name: "without previous and next experiences",
            builder: ExperienceBuilder::new(&Event::new(
                Id::default(),
                "test".to_string().try_into().unwrap(),
                [1, 1].into(),
            )),
            with: vec![],
            result: Ok(()),
        }]
        .into_iter()
        .for_each(|test| {
            let mut constraint = InitialMustBeBeforeOrEqual::new(&test.builder);
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
        });
    }
}
