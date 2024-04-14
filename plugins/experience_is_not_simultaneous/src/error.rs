use plotline_proto::plugin::BeforeSaveExperienceOutput;

#[derive(Debug)]
pub struct Error {
    pub error: String,
    pub details: String,
}

impl Default for Error {
    fn default() -> Self {
        Self::new("an entity cannot experience simultaneous events".to_string())
    }
}

impl From<Error> for BeforeSaveExperienceOutput {
    fn from(value: Error) -> Self {
        BeforeSaveExperienceOutput {
            error: value.error,
            details: value.details,
            ..Default::default()
        }
    }
}

impl Error {
    pub fn new(error: String) -> Self {
        Self {
            error,
            details: Default::default(),
        }
    }

    pub fn with_details(mut self, details: String) -> Self {
        self.details = details;
        self
    }
}
