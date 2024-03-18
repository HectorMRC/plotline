use plotline::event::Event;
use plotline::id::Identifiable;
use plotline_proto::model as proto;

pub(crate) fn proto_event<Intv>(event: &Event<Intv>) -> proto::Event {
    proto::Event {
        id: event.id().to_string(),
        name: event.name.to_string(),
        ..Default::default()
    }
}
