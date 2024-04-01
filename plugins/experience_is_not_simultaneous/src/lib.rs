use plotline_proto::plugin::{
    BeforeSaveExperienceInput, BeforeSaveExperienceOutput, GetPluginKind, GetPluginId, PluginKind,
};
use protobuf::Message;

#[no_mangle]
fn id() -> *const u8 {
    let output = GetPluginId {
        id: "ExperienceIsNotSimultaneous".to_string(),
        ..Default::default()
    };

    let output_bytes = output.write_to_bytes().unwrap();
    let output_len = (output_bytes.len() as u32).to_le_bytes();
    let output_bytes = [&output_len[..], &output_bytes].concat();
    output_bytes.as_ptr()
}

#[no_mangle]
fn kind() -> *const u8 {
    let output = GetPluginKind {
        kind: PluginKind::BEFORE_SAVE_EXPERIENCE.into(),
        ..Default::default()
    };

    let output_bytes = output.write_to_bytes().unwrap();
    let output_len = (output_bytes.len() as u32).to_le_bytes();
    let output_bytes = [&output_len[..], &output_bytes].concat();
    output_bytes.as_ptr()
}

#[no_mangle]
fn run(ptr: u32) -> *const u8 {
    let input = unsafe {
        let len = *(ptr as *const u32);
        let bytes = (ptr + 4) as *const u8;
        let slice = core::slice::from_raw_parts(bytes, len as usize);
        BeforeSaveExperienceInput::parse_from_bytes(slice).unwrap()
    };

    let output = BeforeSaveExperienceOutput {
        error: format!("hardcoded error: {}", input.subject.id),
        ..Default::default()
    };

    let output_bytes = output.write_to_bytes().unwrap();
    let output_len = (output_bytes.len() as u32).to_le_bytes();
    let output_bytes = [&output_len[..], &output_bytes].concat();
    output_bytes.as_ptr()
}

// use super::{Constraint, Error, Recoverable, Result};
// use crate::{error::ResidueError, event::Event, experience::ExperiencedEvent, interval::Interval};

// pub struct ExperienceIsNotSimultaneous<'a, Intv> {
//     event: &'a Event<Intv>,
//     conflict: Option<&'a ExperiencedEvent<'a, Intv>>,
// }

// impl<'a, Intv> Constraint<'a, Intv> for ExperienceIsNotSimultaneous<'a, Intv>
// where
//     Intv: Interval,
// {
//     fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Recoverable<Self> {
//         if self.event.intersects(experienced_event.event) {
//             self.conflict = Some(experienced_event);
//             return Err(ResidueError::new(self, Error::SimultaneousEvents));
//         }

//         Ok(self)
//     }

//     fn result(self) -> Result<()> {
//         if self.conflict.is_some() {
//             return Err(Error::SimultaneousEvents);
//         }

//         Ok(())
//     }
// }

// impl<'a, Intv> ExperienceIsNotSimultaneous<'a, Intv> {
//     pub fn new(event: &'a Event<Intv>) -> Self {
//         Self {
//             event,
//             conflict: None,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::{
//         event::Event,
//         experience::{
//             constraint::{Constraint, Error, ExperienceIsNotSimultaneous, Result},
//             tests::transitive_experience,
//             ExperiencedEvent,
//         },
//         period::Period,
//     };

//     #[test]
//     fn experience_is_not_simultaneous() {
//         struct Test<'a> {
//             name: &'a str,
//             event: Event<Period<usize>>,
//             with: Vec<ExperiencedEvent<'a, Period<usize>>>,
//             result: Result<()>,
//         }

//         vec![
//             Test {
//                 name: "experience without surroundings",
//                 event: Event::fixture([1, 3]),
//                 with: vec![],
//                 result: Ok(()),
//             },
//             Test {
//                 name: "experience with previous",
//                 event: Event::fixture([1, 3]),
//                 with: vec![ExperiencedEvent {
//                     experience: &transitive_experience(),
//                     event: &Event::fixture([0, 0]),
//                 }],
//                 result: Ok(()),
//             },
//             Test {
//                 name: "experience with previous overlapping",
//                 event: Event::fixture([1, 3]),
//                 with: vec![ExperiencedEvent {
//                     experience: &transitive_experience(),
//                     event: &Event::fixture([0, 1]),
//                 }],
//                 result: Err(Error::SimultaneousEvents),
//             },
//             Test {
//                 name: "experience with partial overlapping",
//                 event: Event::fixture([1, 3]),
//                 with: vec![ExperiencedEvent {
//                     experience: &transitive_experience(),
//                     event: &Event::fixture([2, 2]),
//                 }],
//                 result: Err(Error::SimultaneousEvents),
//             },
//             Test {
//                 name: "experience with total overlapping",
//                 event: Event::fixture([1, 3]),
//                 with: vec![ExperiencedEvent {
//                     experience: &transitive_experience(),
//                     event: &Event::fixture([1, 3]),
//                 }],
//                 result: Err(Error::SimultaneousEvents),
//             },
//             Test {
//                 name: "experience with next overlapping",
//                 event: Event::fixture([1, 3]),
//                 with: vec![ExperiencedEvent {
//                     experience: &transitive_experience(),
//                     event: &Event::fixture([3, 4]),
//                 }],
//                 result: Err(Error::SimultaneousEvents),
//             },
//             Test {
//                 name: "experience with next",
//                 event: Event::fixture([1, 3]),
//                 with: vec![ExperiencedEvent {
//                     experience: &transitive_experience(),
//                     event: &Event::fixture([4, 4]),
//                 }],
//                 result: Ok(()),
//             },
//         ]
//         .into_iter()
//         .for_each(|test| {
//             let constraint = ExperienceIsNotSimultaneous::new(&test.event);
//             let result = test
//                 .with
//                 .iter()
//                 .try_fold(constraint, |constraint, experienced_event| {
//                     constraint.with(experienced_event)
//                 })
//                 .map_err(Into::into)
//                 .and_then(|constraint| constraint.result());

//             assert_eq!(
//                 result, test.result,
//                 "{} got = {:?}, want = {:?}",
//                 test.name, result, test.result
//             );
//         });
//     }
// }
