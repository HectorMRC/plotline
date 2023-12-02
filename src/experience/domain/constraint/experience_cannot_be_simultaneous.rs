use crate::experience::ExperienceBuilder;

pub struct ExperienceCannotBeSimultaneous<'a, Intv> {
    builder: ExperienceBuilder<'a, Intv>,
}
