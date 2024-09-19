use futures_lite::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use std::env;
use tracing::info;

fn main() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");

    tracing_subscriber::fmt::init();

    // Config settings.
    let amqp_addr = env::var("AMQP_ADDR").unwrap_or("amqp://127.0.0.1:5672/%2f".into());
    let q = env::var("QUEUE_NAME").unwrap_or("test.qq".into());
    let consumer_name = env::var("CONSUMER_NAME").unwrap_or("rabbitmq_ha_tester_q_reader".into());

    async_global_executor::block_on(async {
        //
        info!("Connecting to {} ...", amqp_addr);
        let conn = Connection::connect(&amqp_addr, ConnectionProperties::default())
            .await
            .expect("Failed to connect to RabbitMQ");
        info!("Sucessfully connected.");

        let channel = conn
            .create_channel()
            .await
            .expect("Failed to open a channel");
        info!("Channel state:{:?}.", channel.status().state());

        let mut consumer = channel
            .basic_consume(
                &q,
                &consumer_name,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");

        info!("Waiting for messages ...");
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                info!(
                    "Received message: {:?}",
                    std::str::from_utf8(&delivery.data).unwrap()
                );
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
    });

    println!("Exiting now.");
}
