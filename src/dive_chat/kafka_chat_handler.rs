use std::sync::{Arc, Mutex};
use std::str;

use kafka::consumer::{Consumer, Message, MessageSet, MessageSets, FetchOffset, GroupOffsetStorage};
use kafka::producer::{Producer, Record};
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::config::FromClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::ClientConfig;
use rdkafka::client::Client;
use serde_json::Value;
use std::time::Duration;

use super::model::UserWithAuthor;

#[derive(Clone)]
pub struct KafkaChatHandler {
    consumer: Arc<Mutex<Consumer>>,
    producer: Arc<Mutex<Producer>>
   }

   impl KafkaChatHandler {
    pub async  fn new(hosts: Vec<String>, topic: String) -> Result<Self, KafkaError> {
      let admin_client = AdminClient::from_config(&ClientConfig::new()
          .set("bootstrap.servers", hosts.join(",")))
          .expect("Failed to create AdminClient");

        // let client = ClientConfig::new()
        // .set("bootstrap.servers", hosts)
        // .create()
        // .expect("Failed to create Kafka client");

      let result = admin_client
      .create_topics(
          &[NewTopic {
              name: &topic.clone(),
              num_partitions: 1,
              replication: TopicReplication::Fixed(1),
              config: vec![],
          }],
          &AdminOptions::default(),
      )
      .await;

     match result {
         Ok(top) => {
          let producer =
          Producer::from_hosts( hosts.clone() )
            .create()
            .unwrap();
    
            let consumer = Consumer::from_hosts(hosts)
            .with_topic(topic)
            .with_group(String::from("new group"))
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()
            .unwrap();
            
            Ok(Self {
              consumer: Arc::from(Mutex::new(consumer)),
              producer: Arc::from(Mutex::new(producer))
          })
         },
         Err(eror) => {
          println!("{:?}", eror.to_string());
          Err(eror)
         }
     }
  }

    pub fn send_message(&mut self, topic: &str, data: UserWithAuthor) {
      let json = serde_json::to_string(&data).unwrap();
      let record = Record::from_value(topic, json);
      let mut producer = self.producer.lock().unwrap();

      producer.send(&record).unwrap();
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