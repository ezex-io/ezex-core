use crate::topic::TopicMessage;
use futures::channel::mpsc::{Receiver, Sender};
use futures::SinkExt;
use futures::StreamExt;
//re-export used parts
pub use redis_stream_bus::{
    bus::StreamBus, client::RedisClient, config::Config as RedisConfig, mock::MockRedisClient,
    stream::Stream,
};

#[async_trait::async_trait]
pub trait RedisBusTrait: Sized + Send + Sync {
    async fn run(
        mut self,
        mut read_rx: Receiver<Stream>,
        mut add_tx: Sender<Stream>,
        mut ack_tx: Sender<Stream>,
    ) {
        log::info!("Starting redis client for {} service.", self.module_name());

        // TODO: move me to ...???
        // if let Err(err) = self
        //     .send_stream(
        //         None,
        //         Box::new(general::module::Started {
        //             module: self.module_name(),
        //             version: self.module_version(),
        //             started_at: chrono::Utc::now().to_string(),
        //             // message: format!("service {} has started.", self.module_name()),
        //         }),
        //         &mut add_tx,
        //     )
        //     .await
        // {
        //     log::error!("Sending error: {:?}", err);
        // }

        loop {
            if let Some(req) = read_rx.next().await {
                log::debug!("Decoding stream {:?}", req);
                match super::stream::decode(&req.fields) {
                    Ok(req_fields) => {
                        log::debug!("Processing {:?}", req_fields);
                        // About corelation_id
                        // If request has a correlation_id we pick it up.
                        //    Otherwise it is same as the stream_id.
                        //
                        let correlation_id = match req_fields.correlation_id {
                            Some(correlation_id) => correlation_id,
                            None => req.id.clone().unwrap(), // It can't be none
                        };

                        match self.process_message(&req.key, &req_fields.message).await {
                            Ok(msgs) => {
                                if let Err(err) =
                                    self.send_streams(correlation_id, msgs, &mut add_tx).await
                                {
                                    log::error!("Sending error: {:?}", err);
                                }
                            }
                            Err(err) => {
                                log::error!("Processing error: {}", err);
                                if let Err(err) = self
                                    .send_stream(
                                        Some(correlation_id),
                                        Box::new(general::internal::Error {
                                            module: self.module_name(),
                                            message: err.to_string(),
                                        }),
                                        &mut add_tx,
                                    )
                                    .await
                                {
                                    log::error!("Sending error: {:?}", err);
                                }
                            }
                        };

                        log::debug!("Acking {:?}", req.id);
                        if let Err(err) = ack_tx.send(req).await {
                            log::error!("Acking error: {:?}", err);
                        }
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                    }
                }
            }
        }
    }

    async fn send_streams(
        &self,
        correlation_id: String,
        msgs: Vec<Box<dyn TopicMessage>>,
        add_tx: &mut Sender<Stream>,
    ) -> anyhow::Result<()> {
        for msg in msgs {
            self.send_stream(Some(correlation_id.clone()), msg, add_tx)
                .await?;
        }
        Ok(())
    }

    async fn send_stream(
        &self,
        correlation_id: Option<String>,
        msg: Box<dyn TopicMessage>,
        add_tx: &mut Sender<Stream>,
    ) -> anyhow::Result<()> {
        let fields = super::stream::StreamFields {
            module: self.module_name(),
            correlation_id,
            message: msg.to_json_string()?,
        };
        let res = Stream::new(msg.topic(), None, fields.encode()?);
        log::info!("Sending {:?}", res);
        add_tx.send(res).await?;
        Ok(())
    }

    async fn process_message(
        &mut self,
        key: &str,
        msg: &str,
    ) -> anyhow::Result<Vec<Box<dyn TopicMessage>>>;
    fn module_name(&self) -> String;
    fn module_version(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::RedisBusTrait;
    use crate::topic::TopicMessage;
    use async_std::task;
    use futures::{channel::mpsc::channel, SinkExt, StreamExt};
    use procedural::Topic;
    use redis_stream_bus::client::Stream;
    use serde::{Deserialize, Serialize};

    struct TestStreamBus;

    #[derive(Serialize, Deserialize, Debug, Clone, Topic, PartialEq, Eq)]
    #[topic_name("test:test:test")]
    pub struct TestMessage {
        message: String,
    }

    #[async_trait::async_trait]
    impl RedisBusTrait for TestStreamBus {
        async fn process_message(
            &mut self,
            _key: &str,
            _msg: &str,
        ) -> anyhow::Result<Vec<Box<dyn TopicMessage>>> {
            Ok(vec![Box::new(TestMessage {
                message: "hello_world".to_string(),
            })])
        }
        fn module_name(&self) -> String {
            "test".to_string()
        }
        fn module_version(&self) -> String {
            "1.0.0".to_string()
        }
    }

    #[tokio::test]
    async fn test_corelation_id_none() {
        let bus = TestStreamBus;
        let (mut read_tx, read_rx) = channel(100);
        let (add_tx, mut add_rx) = channel(100);
        let (ack_tx, mut ack_rx) = channel(100);
        task::spawn(async move {
            bus.run(read_rx, add_tx, ack_tx).await;
        });

        let req = TestMessage {
            message: "hi".to_string(),
        };
        let fields = crate::stream::StreamFields {
            module: "test".to_string(),
            correlation_id: None,
            message: req.to_json_string().unwrap(),
        };

        let stream_id = Some("stream_id".to_string());
        let req_stream = Stream::new(req.topic(), stream_id.clone(), fields.encode().unwrap());
        read_tx.send(req_stream.clone()).await.unwrap();
        add_rx.next().await; //ignore starting up event
        if let Some(res_stream) = add_rx.next().await {
            let res_fields = crate::stream::decode(&res_stream.fields).unwrap();
            assert_eq!(res_stream.id, None);
            assert_eq!(res_fields.correlation_id, stream_id);
            assert_eq!(
                res_fields.message,
                "{\"message\":\"hello_world\"}".to_string()
            );
        };
        if let Some(ack_stream) = ack_rx.next().await {
            assert_eq!(ack_stream.id, req_stream.id);
        }
    }

    #[tokio::test]
    async fn test_corelation_id_some() {
        let bus = TestStreamBus;
        let (mut read_tx, read_rx) = channel(100);
        let (add_tx, mut add_rx) = channel(100);
        let (ack_tx, _) = channel(100);
        task::spawn(async move {
            bus.run(read_rx, add_tx, ack_tx).await;
        });

        let req = TestMessage {
            message: "hi".to_string(),
        };
        let correlation_id = Some("some_id".to_string());
        let fields = crate::stream::StreamFields {
            module: "test".to_string(),
            correlation_id: correlation_id.clone(),
            message: req.to_json_string().unwrap(),
        };

        let req_stream = Stream::new(
            req.topic(),
            Some("stream_id".to_string()),
            fields.encode().unwrap(),
        );
        read_tx.send(req_stream.clone()).await.unwrap();
        add_rx.next().await; //ignore starting up event
        if let Some(res_stream) = add_rx.next().await {
            let res_fields = crate::stream::decode(&res_stream.fields).unwrap();

            assert_eq!(res_fields.correlation_id, correlation_id);
        };
    }
}
