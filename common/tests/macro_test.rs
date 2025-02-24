use common::topic::TopicMessage;
use procedural::Topic;
use serde::{Deserialize, Serialize};

#[test]
fn test_stream_name() {
    #[derive(Debug, Serialize, Deserialize, Topic)]
    #[topic_name("foo:bar")]
    pub struct Foo {}

    let f = Foo {};

    assert_eq!(f.topic(), "foo:bar");
    assert_eq!(Foo::name, "foo:bar");
}
