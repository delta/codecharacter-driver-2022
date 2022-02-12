use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions};

use crate::request::GameRequest;

fn consumer() -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages...");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body_str = String::from_utf8_lossy(&delivery.body);
                let res: Result<GameRequest, serde_json::Error> = serde_json::from_str(&body_str);
                match res {
                    Ok(match_request) => {
                        println!("i={}, request:  {:?}", i, match_request);
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
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
