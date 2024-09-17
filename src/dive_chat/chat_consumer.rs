use std::sync::{Arc, Mutex};
use std::str;

use kafka::{client::GroupOffsetStorage, consumer::{Consumer, FetchOffset, Message, MessageSet, MessageSets}};
use serde_json::Value;

#[derive(Clone)]
pub struct ChatConsumer {
    consumer: Arc<Mutex<Consumer>>
   }

   impl ChatConsumer {
    pub fn new(hosts: Vec<String>, topic: String) -> Self {
        let consumer = Consumer::from_hosts(hosts)
        .with_topic(topic)
        .with_group(String::from("new group"))
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();
        
        Self {
            consumer: Arc::from(Mutex::new(consumer))
        }
    }

    pub fn get_new_messages_by_chat_id(m: &Message) -> Value {
        let event = str::from_utf8(m.value).unwrap().to_string();
        serde_json::from_str(&event).unwrap()
    }

    pub fn consume_events(&mut self) -> MessageSets {
        let mut consumer = self.consumer.lock().unwrap();
        consumer.poll().unwrap()
      }
     
      pub fn consume_messageset(&mut self, ms: MessageSet) {
        let mut consumer = self.consumer.lock().unwrap();
        consumer.consume_messageset(ms).unwrap();
      }
     
      pub fn commit_consumed(&mut self) {
        let mut consumer = self.consumer.lock().unwrap();
        consumer.commit_consumed().unwrap();
      }
   }