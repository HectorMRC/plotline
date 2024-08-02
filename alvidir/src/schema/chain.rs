use super::Schema;

/// A succession of [Schema]s that must be satisfied as a whole.
pub struct LiFoSchemaChain<Schm, Head> {
    head: Option<Head>,
    schema: Schm,
}

impl<Schm, Head> Schema for LiFoSchemaChain<Schm, Head>
where
    Head: Schema<Source = Schm::Source, Error = Schm::Error>,
    Schm: Schema,
{
    type Source = Schm::Source;
    type Error = Schm::Error;

    fn matches(&self, source: &Self::Source) -> bool {
        self.schema.matches(source)
            && self
                .head
                .as_ref()
                .map(|schema| schema.matches(source))
                .unwrap_or(true)
    }

    fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error> {
        self.schema
            .must_match(source)
            .and_then(|source| match &self.head {
                Some(schema) => schema.must_match(source),
                None => Ok(source),
            })
    }
}

impl<Schm, Head> LiFoSchemaChain<Schm, Head>
where
    Head: Schema<Source = Schm::Source, Error = Schm::Error>,
    Schm: Schema,
{
    /// Chains the given schema with self.
    pub fn chain<Tail>(self, schema: Tail) -> LiFoSchemaChain<Tail, Self>
    where
        Tail: Schema<Source = Schm::Source, Error = Schm::Error>,
    {
        LiFoSchemaChain {
            head: Some(self),
            schema,
        }
    }
}
