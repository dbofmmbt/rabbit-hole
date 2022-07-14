use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let conn =
        Connection::connect("amqp://0.0.0.0:5672/%2f", ConnectionProperties::default()).await?;

    info!("Connected to rabbit");

    let channel = conn.create_channel().await?;

    let queue = channel
        .queue_declare(
            "number",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!(?queue, "Queue declared");

    for number in 0..10 {
        info!(%number, "Producing value");
        channel
            .basic_publish(
                "",
                "number",
                BasicPublishOptions::default(),
                &[number],
                BasicProperties::default(),
            )
            .await?;
    }

    Ok(())
}
