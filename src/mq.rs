use std::sync::{Arc, Mutex};

use crate::{error::SimulatorError, request::GameRequest, response::GameStatus};
use amiquip::{
    Channel, Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions,
    Result,
};

const NUM_OF_THREADS: usize = 2;

pub fn consumer(
    url: String,
    consumer_queue_name: String,
    response_producer_queue_name: String,
    handler_fn: fn(crossbeam_channel::Receiver<GameRequest>, Arc<Publisher>) -> (),
) -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open(&url)?;

    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare(
        &consumer_queue_name,
        QueueDeclareOptions {
            durable: true,
            ..Default::default()
        },
    )?;

    let consumer = queue.consume(ConsumerOptions::default())?;

    let response_publisher = Arc::new(Publisher::new(url, response_producer_queue_name).unwrap());

    let (s, r) = crossbeam_channel::bounded(NUM_OF_THREADS);

    // each thread has a receiver
    let mut threads = vec![];
    for _ in 0..NUM_OF_THREADS {
        let new_r = r.clone();
        let publisher_clone = Arc::clone(&response_publisher);
        threads.push(std::thread::spawn(move || {
            handler_fn(new_r, publisher_clone)
        }))
    }

    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body_str = String::from_utf8_lossy(&delivery.body);
                let res: Result<GameRequest, serde_json::Error> = serde_json::from_str(&body_str);
                match res {
                    Ok(match_request) => {
                        s.send(match_request).unwrap();
                        consumer.ack(delivery)?;
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}

pub struct Publisher {
    connection: Option<Connection>,
    channel: Mutex<Channel>,
    queue_name: String,
}

impl Publisher {
    pub fn new(url: String, queue_name: String) -> Result<Self, SimulatorError> {
        let mut connection = Connection::insecure_open(&url).map_err(|e| {
            SimulatorError::UnidentifiedError(format!(
                "Error in opening connection to publish queue [Connection::insecure_open]: {}",
                e
            ))
        })?;

        let channel = connection.open_channel(None).map_err(|e| {
            SimulatorError::UnidentifiedError(format!(
                "Error in opening channel [Connection::open_channel]: {}",
                e
            ))
        })?;

        channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
            )
            .map_err(|e| {
                SimulatorError::UnidentifiedError(format!(
                    "Error in publishing to the queue [Publisher::new]: {}",
                    e
                ))
            })?;

        Ok(Self {
            connection: Some(connection),
            channel: Mutex::new(channel),
            queue_name,
        })
    }
    pub fn publish(&self, response: GameStatus) -> Result<(), SimulatorError> {
        let channel = self.channel.lock().unwrap();
        let exchange = Exchange::direct(&channel);
        let body = serde_json::to_string(&response)
            .map_err(|e| SimulatorError::UnidentifiedError(format!("{}", e)))?;
        exchange
            .publish(Publish::new(&body.as_bytes(), &self.queue_name))
            .map_err(|e| {
                SimulatorError::UnidentifiedError(format!(
                    "Error in publishing to the queue[Publisher::publish]{}",
                    e
                ))
            })?;
        Ok(())
    }
}

impl Drop for Publisher {
    fn drop(&mut self) {
        if self.connection.is_some() {
            let conn = self.connection.take().unwrap();
            let _ = conn.close();
        }
    }
}
