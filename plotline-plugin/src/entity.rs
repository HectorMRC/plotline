use plotline::entity::Entity;
use plotline::id::Indentify;
use plotline_proto::model as proto;

pub(crate) fn proto_entity(entity: &Entity) -> proto::Entity {
    proto::Entity {
        id: entity.id().to_string(),
        name: entity.name.to_string(),
        ..Default::default()
    }
}
