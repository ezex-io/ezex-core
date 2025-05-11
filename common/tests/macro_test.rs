use common::event::EventMessage;
use procedural::Event;
use serde::{
    Deserialize,
    Serialize,
};

#[test]
fn test_stream_name() {
    #[derive(Debug, Serialize, Deserialize, Event)]
    #[event_key("foo:bar")]
    pub struct Foo {}

    let f = Foo {};

    assert_eq!(f.key(), "foo:bar");
    assert_eq!(Foo::name, "foo:bar");
}
