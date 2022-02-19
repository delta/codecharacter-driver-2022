use crate::{error::SimulatorError, request::GameRequest, response::GameStatus};
use amiquip::{
    Channel, Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions,
    Result,
};

pub fn consumer(
    url: String,
    consumer_queue_name: String,
    response_producer_queue_name: String,
    handler_fn: fn(GameRequest, &mut Publisher) -> (),
) -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open(&url)?;

    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare(&consumer_queue_name, QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;

    let mut response_publisher = Publisher::new(url, response_producer_queue_name).unwrap();

    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body_str = String::from_utf8_lossy(&delivery.body);
                let res: Result<GameRequest, serde_json::Error> = serde_json::from_str(&body_str);
                match res {
                    Ok(match_request) => {
                        consumer.ack(delivery)?;
                        handler_fn(match_request, &mut response_publisher);
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
    channel: Channel,
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
            .queue_declare(&queue_name, QueueDeclareOptions::default())
            .map_err(|e| {
                SimulatorError::UnidentifiedError(format!(
                    "Error in publishing to the queue [Publisher::new]: {}",
                    e
                ))
            })?;

        Ok(Self {
            connection: Some(connection),
            channel,
            queue_name,
        })
    }
    pub fn publish(&mut self, response: GameStatus) -> Result<(), SimulatorError> {
        let exchange = Exchange::direct(&self.channel);
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
