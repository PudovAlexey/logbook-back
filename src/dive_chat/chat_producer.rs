use kafka::producer::{Producer, Record};
use serde_json;

use super::model::Message;

pub struct ChatProducer {
    producer: Producer
}

impl ChatProducer {
   pub fn new(hosts: Vec<String>) -> Self {
        let producer =
      Producer::from_hosts( hosts )
        .create()
        .unwrap();

    Self {
        producer
    }
    }

    pub fn send_message(&mut self, topic: &str, data: Message) {
        let json = serde_json::to_string(&data).unwrap();
        let record = Record::from_value(topic, json);

        self.producer.send(&record).unwrap();
    }
}