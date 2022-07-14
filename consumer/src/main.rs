use futures_lite::stream::StreamExt;
use lapin::{options::BasicConsumeOptions, types::FieldTable, Connection, ConnectionProperties};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let conn =
        Connection::connect("amqp://0.0.0.0:5672/%2f", ConnectionProperties::default()).await?;

    info!("Connected to rabbit");

    let channel = conn.create_channel().await?;

    info!("Channel created");

    let mut consumer = channel
        .basic_consume(
            "number",
            "the_consumer",
            BasicConsumeOptions {
                no_ack: true,
                ..default()
            },
            FieldTable::default(),
        )
        .await?;

    info!("Starting consume...");

    while let Some(message) = consumer.next().await {
        match message {
            Ok(delivery) => info!(data=?delivery.data, "Message received"),
            Err(error) => warn!(%error, "There was a problem with the delivery"),
        }
    }

    Ok(())
}

fn default<T: Default>() -> T {
    T::default()
}
