use anyhow::{anyhow, Ok, Result};
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;

use log::info;

use crate::word_engine::service::WordEngine;

pub struct KafkaEngine {
    pub uri: String,
    pub topic: String,
    group_id: String,
    producer: Producer,
    consumer: Consumer,
    word_engine: WordEngine,
}

impl KafkaEngine {
    pub fn new(uri: String, topic: String, group_id: String, partitions: Vec<i32>, random_word_generator_api_url: String) -> Result<Self> {
        

        let consumer = Consumer::from_hosts(vec!(uri.to_owned()))
          .with_topic_partitions(topic.to_owned(), &partitions)
          .with_fallback_offset(FetchOffset::Earliest)
          .with_group(group_id.to_owned())
          .with_offset_storage(GroupOffsetStorage::Kafka)
          .create()
          .map_err(|e| anyhow!("Failed to create kafka consumer kafka with uri: {}, topic: {}, group_id: {}, error: {}", uri, topic, group_id, e))?;


          let producer = Producer::from_hosts(vec![uri.to_owned()])
          .with_ack_timeout(Duration::from_secs(1))
          .with_required_acks(RequiredAcks::One)
          .create()
          .map_err(|e| {
              anyhow!(
                  "Failed to instantiated kafka producer with uri: {}, error: {}",
                  uri,
                  e
              )
          })?;

          let word_engine = WordEngine::new(random_word_generator_api_url);
          
        Ok(Self {
            uri,
            producer,
            consumer,
            topic,
            group_id,
            word_engine
        })
    }

    pub fn send_message(&mut self, message: String) -> Result<()> {
        self.producer
            .send(&Record::from_value(self.topic.as_str(), message.as_bytes()))
            .map_err(|e| {
                anyhow!(
                    "Failed to send message with uri: {}, topic: {}, group_id: {}, error: {}",
                    self.uri,
                    self.topic,
                    self.group_id,
                    e
                )
            })?;
        Ok(())
    }

    
    pub async fn consumes_messages_and_send_sentence(&mut self) -> Result<()> {
        info!(
            "Waiting messages with uri: {}, topic {}, group_id: {}",
            self.uri, self.topic, self.group_id
        );
        loop {
            for ms in self.consumer.poll().unwrap().iter() {
                for m in ms.messages() {

                  let value = String::from_utf8(m.value.to_vec())
                  .map_err(|e| anyhow!("Failed to parse String from message
                   {:?},: {}", m , e))?;

                  info!(
                    "Receiving message : {}", value
                  );

                  let message = self.word_engine.get_word_or_dot().await;
                  self.send_message(message)?;
                }
            }
            self.consumer.commit_consumed().unwrap();
        }
    }


}
