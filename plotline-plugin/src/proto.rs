//! All the conversions between protobuf and the domain.

use crate::{Error, Result};
use plotline::id::Indentify;
use plotline::interval::{Interval, IntervalFactory};
use plotline::plugin::OutputError;
use plotline::{
    entity::Entity,
    event::Event,
    experience::{Experience, Profile},
};
use std::collections::HashMap;
use std::fmt::Display;

mod protobuf {
    pub use plotline_proto::model::*;
    pub use plotline_proto::plugin::*;
    pub use protobuf::MessageField;
}

/// Returns the proto message for the given [Entity].
pub fn from_entity(entity: &Entity) -> protobuf::Entity {
    protobuf::Entity {
        id: entity.id().to_string(),
        name: entity.name.to_string(),
        ..Default::default()
    }
}

/// Returns the [Entity] contained in the proto message.
pub fn into_entity(entity: &protobuf::Entity) -> Result<Entity> {
    Ok(Entity {
        id: entity.id.clone().try_into()?,
        name: entity.name.clone().try_into()?,
    })
}

/// Returns the proto message for the given [Interval].
pub fn from_interval<Intv>(interval: &Intv) -> protobuf::Interval
where
    Intv: Interval,
    Intv::Bound: Display,
{
    protobuf::Interval {
        lo: interval.lo().to_string(),
        hi: interval.hi().to_string(),
        ..Default::default()
    }
}

/// Returns the [Interval] contained in the proto message.
pub fn into_interval<Intv>(interval: &protobuf::Interval) -> Result<Intv>
where
    Intv: IntervalFactory,
    Intv::Bound: TryFrom<String>,
    <Intv::Bound as TryFrom<String>>::Error: Display,
{
    let parse_boundary = |value: String| -> Result<Intv::Bound> {
        let bound: std::result::Result<Intv::Bound, _> = value.try_into();
        bound.map_err(|err| Error::Interval(err.to_string()))
    };

    Ok(Intv::new(
        parse_boundary(interval.lo.clone())?,
        parse_boundary(interval.hi.clone())?,
    ))
}

/// Returns the proto message for the given [Event].
pub fn from_event<Intv>(event: &Event<Intv>) -> protobuf::Event
where
    Intv: Interval,
    Intv::Bound: Display,
{
    protobuf::Event {
        id: event.id().to_string(),
        name: event.name.to_string(),
        interval: protobuf::MessageField::some(from_interval(&event.interval)),
        ..Default::default()
    }
}

/// Returns the [Event] contained in the proto message.
pub fn into_event<Intv>(event: &protobuf::Event) -> Result<Event<Intv>>
where
    Intv: IntervalFactory,
    Intv::Bound: TryFrom<String>,
    <Intv::Bound as TryFrom<String>>::Error: Display,
{
    let Some(proto_interval) = &event.interval.0 else {
        return Err(Error::MissingField("interval"));
    };

    Ok(Event {
        id: event.id.clone().try_into()?,
        name: event.name.clone().try_into()?,
        interval: into_interval(proto_interval)?,
    })
}

/// Returns the proto message for the given [Profile].
pub fn from_profile(profile: &Profile) -> protobuf::Profile {
    protobuf::Profile {
        entity: protobuf::MessageField::some(from_entity(&profile.entity)),
        values: profile
            .values
            .iter()
            .map(|(key, value)| protobuf::KeyValue {
                key: key.to_string(),
                value: value.to_string(),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    }
}

/// Returns the [Profile] contained in the proto message.
pub fn into_profile(profile: &protobuf::Profile) -> Result<Profile> {
    let Some(proto_entity) = &profile.entity.0 else {
        return Err(Error::MissingField("entity"));
    };

    Ok(Profile {
        entity: into_entity(proto_entity)?,
        values: HashMap::from_iter(
            profile
                .values
                .iter()
                .cloned()
                .map(|item| (item.key, item.value)),
        ),
    })
}

/// Returns the proto message for the given [Experience].
pub fn from_experience<Intv>(experience: &Experience<Intv>) -> protobuf::Experience
where
    Intv: Interval,
    Intv::Bound: Display,
{
    protobuf::Experience {
        id: experience.id().to_string(),
        entity: protobuf::MessageField::some(from_entity(&experience.entity)),
        event: protobuf::MessageField::some(from_event(&experience.event)),
        profiles: experience.profiles.iter().map(from_profile).collect(),
        ..Default::default()
    }
}

/// Returns the [Experience] contained in the proto message.
pub fn into_experience<Intv>(experience: &protobuf::Experience) -> Result<Experience<Intv>>
where
    Intv: IntervalFactory,
    Intv::Bound: TryFrom<String>,
    <Intv::Bound as TryFrom<String>>::Error: Display,
{
    let Some(proto_entity) = &experience.entity.0 else {
        return Err(Error::MissingField("entity"));
    };

    let Some(proto_event) = &experience.event.0 else {
        return Err(Error::MissingField("event"));
    };

    Ok(Experience {
        id: experience.id.clone().try_into()?,
        entity: into_entity(proto_entity)?,
        event: into_event(proto_event)?,
        profiles: experience
            .profiles
            .iter()
            .map(into_profile)
            .collect::<Result<Vec<_>>>()?,
    })
}

/// Returns the proto message for the given [PluginError].
pub fn from_error(error: &OutputError) -> protobuf::PluginError {
    protobuf::PluginError {
        code: error.code.clone(),
        message: error.message.clone(),
        ..Default::default()
    }
}
