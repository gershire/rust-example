use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_stream::StreamExt;

pub async fn listen(bootstrap_server: &str, topic: &str) -> Receiver<String> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "app")
        .set("bootstrap.servers", bootstrap_server)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Consumer creation failed");
    consumer.subscribe(&[&topic]).expect("Can't subscribe to specified topic");

    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        println!("[LISTENER] Listening...");
        let mut stream = consumer.stream();
        while let Some(res) = stream.next().await {
            println!("[LISTENER] Got something from Kafka");
            match res {
                Ok(borrowed_message) => {
                    let owned_message = borrowed_message.detach();
                    println!("[LISTENER] Offset: {}", owned_message.offset());
                    match owned_message.payload_view::<str>() {
                        Some(Ok(payload)) => tx.send(payload.to_string()).await.unwrap(),
                        Some(Err(_)) => println!("[LISTENER] Message payload is not a string!"),
                        None => println!("[LISTENER] No payload")
                    }
                }
                Err(e) => println!("[LISTENER] Kafka error: {}", e)
            }
        }
    });
    rx
}
