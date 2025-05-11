use erased_serde::Serialize;
use std::{any::Any, fmt::Debug};

pub trait EventMessage: Debug + Send + Sync + Serialize {
    fn key(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

#[cfg(test)]
mod test {
    use super::*;
    use procedural::Event;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug, Event, PartialEq)]
    #[event_key("test-key")]
    pub struct TestEvent {
        pub msg: String,
    }

    #[test]
    fn test_event_key() {
        let evt = TestEvent {
            msg: "hello_world".to_string(),
        };

        assert_eq!(evt.key(), "test-key".to_string());
    }

    #[test]
    fn test_event_serde_redis() {
        let evt1 = TestEvent {
            msg: "hello_world".to_string(),
        };

        let ser = serde_redis::Serializer;
        let val = evt1.serialize(ser).unwrap();

        let de = serde_redis::Deserializer::new(&val);
        let evt2: TestEvent = Deserialize::deserialize(de).unwrap();

        assert_eq!(evt1, evt2);
    }

    #[test]
    fn test_event_boxed_serde_json() {
        let evt1 = TestEvent {
            msg: "hello_world".to_string(),
        };

        let boxed_event: Box<dyn EventMessage> = Box::new(evt1.clone());

        let mut str = vec![];
        let mut serializer = serde_json::Serializer::new(&mut str);
        boxed_event
            .erased_serialize(&mut <dyn erased_serde::Serializer>::erase(&mut serializer))
            .unwrap();

        let mut deserializer = serde_json::Deserializer::from_slice(&str);
        let evt2: TestEvent = erased_serde::deserialize(
            &mut <dyn erased_serde::Deserializer>::erase(&mut deserializer),
        )
        .unwrap();

        assert_eq!(evt1, evt2);
    }

    // #[test]
    // fn test_event_boxed_serde_redis() {
    //     let evt1 = TestEvent {
    //         msg: "hello_world".to_string(),
    //     };

    //     let boxed_event: Box<dyn EventMessage> = Box::new(evt1.clone());
    //     let serializer = serde_redis::Serializer;

    //     let mut formatter = <dyn erased_serde::Serializer>::erase(serializer);
    //     let ok = boxed_event.erased_serialize(&mut formatter).unwrap();

    //     // let value = ok.into();

    //     let mut deserializer = serde_redis::Deserializer::new(value);
    //     let evt2: TestEvent =
    //         erased_serde::deserialize(&mut <dyn erased_serde::Deserializer>::erase(deserializer))
    //             .unwrap();

    //     assert_eq!(evt1, evt2);
    // }
}
