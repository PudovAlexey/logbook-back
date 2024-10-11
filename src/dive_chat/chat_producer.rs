use std::sync::{Arc, Mutex};
use kafka::producer::{Producer, Record};
use serde_json;

use super::model::{Message, UserWithAuthor};

#[derive(Clone)]
pub struct ChatProducer {
    producer: Arc<Mutex<Producer>>,
}

impl ChatProducer {
   pub fn new(hosts: Vec<String>) -> Self {
        let producer =
      Producer::from_hosts( hosts )
        .create()
        .unwrap();

    let result = Arc::from(Mutex::new(producer));

    Self {
        producer: result
    }
    }

    pub fn send_message(&mut self, topic: &str, data: UserWithAuthor) {
        let json = serde_json::to_string(&data).unwrap();
        let record = Record::from_value(topic, json);
        let mut producer = self.producer.lock().unwrap();

        producer.send(&record).unwrap();
        // let producer = Arc::try_unwrap(self.producer.clone());

        // match producer {
        //     Ok(mut producer) => {
        //         producer.send(&record).unwrap();
        //     },
        //     Err(error) => {
        //         println!("error in sending message in producer")
        //     }
        // }
    }
}