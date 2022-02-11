use serde_json;

use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};

use crate::request;

fn consumer() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages...");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body_str = String::from_utf8_lossy(&delivery.body);
                let res = serde_json::from_str(&body_str);
                if res.is_ok() {
                    let mat : request::MatchRequest = res.unwrap();
                    println!("({:>3}) Received msg : {}", i, mat.map);
                }
                else{
                    println!("Err:");
                    println!("{:#?}",res.err())
                }
                
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
