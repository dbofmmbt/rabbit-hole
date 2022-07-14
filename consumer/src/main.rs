use std::time::Duration;

use anyhow::anyhow;
use futures_lite::stream::StreamExt;
use lapin::{options::BasicConsumeOptions, types::FieldTable, Connection, ConnectionProperties};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let rabbit_host = std::env::var("RABBIT_HOST").or_else(|error| match error {
        std::env::VarError::NotPresent => {
            let default_host = "0.0.0.0";
            warn!(default=%default_host, "RABBIT_HOST env var not set");
            Ok(default_host.into())
        }
        _ => Err(error),
    })?;

    let conn_url = format!("amqp://{rabbit_host}:5672/%2f");

    let conn = {
        let mut tries = 0;
        loop {
            let result = Connection::connect(&conn_url, ConnectionProperties::default()).await;
            if let Ok(conn) = result {
                break conn;
            }

            tries += 1;
            if tries >= 10 {
                return Err(anyhow!(
                    "couldn't connect to rabbit even after several tries"
                ));
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    };

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
