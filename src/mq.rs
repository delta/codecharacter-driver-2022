use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result, Publish, Exchange, AmqpProperties};
use std::{borrow::{Cow}};
use crate::request::GameRequest;

fn consumer() -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;

    let exchange = Exchange::direct(&channel);

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
                let reply_to = delivery.properties.reply_to().as_ref().unwrap().clone();
                channel.basic_publish(exchange.name(), Publish::new("accepted".as_bytes(), reply_to))?;
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

pub fn publish() -> Result<String> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    channel.queue_declare("hello", QueueDeclareOptions::default())?;

    let exchange = Exchange::direct(&channel);

    let props = AmqpProperties::default().with_reply_to(String::from("bye"));

    exchange.publish(Publish::with_properties(r#"{"game_id":"0fa0f12d-d472-42d5-94b4-011e0c916023","parameters":{"attackers":[{"id":1,"hp":10,"range":3,"attack_power":3,"speed":3,"price":1},{"id":2,"hp":10,"range":3,"attack_power":3,"speed":3,"price":1}],"defenders":[{"id":1,"hp":10,"range":4,"attack_power":5,"price":1},{"id":2,"hp":10,"range":6,"attack_power":5,"price":1}],"no_of_turns":500,"no_of_coins":1000},"source_code":"print(x)","language":"PYTHON","map":"[[1,0],[0,2]]"}"#.as_bytes(), "hello", props))?;
    
    let queue = channel.queue_declare("bye", QueueDeclareOptions::default())?;
    let message = queue.consume(ConsumerOptions::default()).unwrap().receiver().recv().unwrap();
    let body;
    match message {
        ConsumerMessage::Delivery(delivery) => {
            body = String::from_utf8_lossy(&delivery.body).into_owned();
            println!("({:>3}) Received", body);
        }
        other => {
            body=Cow::Borrowed("error").into_owned();
            println!("Consumer ended: {:?}", other);
        }
    }
    connection.close();
    return Ok(body);
}
