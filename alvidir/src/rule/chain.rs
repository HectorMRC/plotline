//! A rule implementation for statically chaining arbitrary rules.

use std::marker::PhantomData;

use super::Rule;

/// A succession of arbitrary [Rule]s that must be satisfied as a single one.
pub struct LiFoRuleChain<Cnst, Head> {
    rule: Cnst,
    head: Head,
}

impl<Cnst, Head> Rule for LiFoRuleChain<Cnst, Head>
where
    Head: Rule<Source = Cnst::Source, Error = Cnst::Error>,
    Cnst: Rule,
{
    type Source = Cnst::Source;
    type Error = Cnst::Error;

    fn matches(&self, source: &Self::Source) -> bool {
        self.rule.matches(source) && self.head.matches(source)
    }

    fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error> {
        self.rule
            .must_match(source)
            .and_then(|source| self.head.must_match(source))
    }
}

impl<Cnst, Head> LiFoRuleChain<Cnst, Head>
where
    Head: Rule<Source = Cnst::Source, Error = Cnst::Error>,
    Cnst: Rule,
{
    /// Chains the given rule with self.
    pub fn chain<Tail>(self, rule: Tail) -> LiFoRuleChain<Tail, Self>
    where
        Tail: Rule<Source = Cnst::Source, Error = Cnst::Error>,
    {
        LiFoRuleChain {
            rule,
            head: self,
        }
    }
}

impl<Cnst> LiFoRuleChain<Cnst, InfallibleRule<Cnst::Source, Cnst::Error>>
where
    Cnst: Rule,
{
    /// Creates a new constrain chain with the given one, having [InfallibleRule] as the head
    /// of self.
    pub fn new(rule: Cnst) -> Self {
        Self {
            head: Default::default(),
            rule,
        }
    }
}

/// A [Rule] implementation that never fails.
pub struct InfallibleRule<Src, Err> {
    source: PhantomData<Src>,
    error: PhantomData<Err>,
}

impl<Src, Err> Default for InfallibleRule<Src, Err> {
    fn default() -> Self {
        Self {
            source: PhantomData,
            error: PhantomData,
        }
    }
}

impl<Src, Err> Rule for InfallibleRule<Src, Err> {
    type Source = Src;
    type Error = Err;

    fn matches(&self, _: &Self::Source) -> bool {
        true
    }

    fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error> {
        Ok(source)
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::{fixtures::RuleMock, Rule};

    use super::LiFoRuleChain;

    fn must_contain<const C: char>() -> RuleMock<&'static str, &'static str> {
        RuleMock::default()
            .with_matches_fn(|s: &&str| s.contains(C))
            .with_must_match_fn(|s| {
                if !s.contains(C) {
                    return Err("does not contains expected char");
                }

                Ok(s)
            })
    }

    #[test]
    fn lifo_rule_chain_must_run_all_rules() {
        let rule = LiFoRuleChain::new(must_contain::<'a'>())
            .chain(must_contain::<'1'>())
            .chain(must_contain::<'ش'>());

        struct Test {
            name: &'static str,
            subject: &'static str,
            matches: bool,
        }

        vec![
            Test {
                name: "subject failing all rules should fail",
                subject: "hello world",
                matches: false,
            },
            Test {
                name: "subject failing one single rule should fail",
                subject: "a1",
                matches: false,
            },
            Test {
                name: "subject fulfilling all rules should success",
                subject: "a1ش",
                matches: true,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let matches = rule.matches(&test.subject);
            assert_eq!(
                matches, test.matches,
                "{} got matches = {matches}, want {}",
                test.name, test.matches
            )
        })
    }
}
