use std::io;
use std::sync::Arc;
use rocksdb::DB;
use crate::data_model::Location;
use crate::vehicle_service_grpc::VehicleService;
use log::debug;
use log::error;

#[path = "configuration/init_config.rs"]
mod init_config;
#[path = "configuration/settings.rs"]
mod settings;
mod kafka_consumer;
mod data_model;
mod vehicle_service_grpc;

#[tokio::main]
async fn main() {
    let settings = init_config::load_config();
    let path = settings.database.path;
    let db = Arc::new(DB::open_default(path).unwrap());

    let bootstrap_server = settings.kafka.bootstrap_server;
    let topic = settings.kafka.topic;
    let mut rx = kafka_consumer::listen(&bootstrap_server, &topic).await;

    let dbr = db.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            handle_message(msg, dbr.as_ref()).await;
        }
    });

    let addr = settings.grpc.socket_address.parse()
        .expect("couldn't parse socket address");
    VehicleService::start(db.clone(), addr).await;
}

async fn handle_message(message: String, db: &rocksdb::DB) {
    debug!("Handling message {}", message);
    let res = serde_json::from_str::<data_model::Vehicle>(&*message);
    match res {
        Ok(vehicle) => {
            db.put(vehicle.id, vehicle.location);
        }
        Err(e) => error!("Message handling error: {}", e)
    }
}
