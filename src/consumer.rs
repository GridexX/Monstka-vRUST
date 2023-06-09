use futures::stream::Stream;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use std::boxed::Box;
use tokio::runtime::current_thread::Runtime;

mod utils;

fn echo_message<M: Message>(msg: M) -> Result<(), std::str::Utf8Error> {
    let deserialize = |o| match o {
        None => Ok(""),
        Some(val) => Ok(std::str::from_utf8(val)?),
    };

    println!(
        "Consumed record from topic {} partition [{}] @ offset {} with key {} and value {}",
        msg.topic(),
        msg.partition(),
        msg.offset(),
        deserialize(msg.key())?,
        deserialize(msg.payload())?,
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (topic, mut config) = utils::get_config()?;
    let consumer: StreamConsumer = config.set("group.id", "rust_example_group_1").create()?;

    consumer.subscribe(&vec![topic.as_ref()])?;

    let processor = consumer
        .start()
        .filter_map(|result| match result {
            Ok(_) => result.ok(),
            Err(err) => {
                eprintln!("error consuming from message stream: {}", err);
                None
            }
        })
        .for_each(|msg| echo_message(msg).map_err(|_| eprintln!("error deserializing message")));

    Runtime::new()?
        .block_on(processor)
        .map_err(|_| eprintln!("error running consumer on current thread"))
        .ok();

    Ok(())
}