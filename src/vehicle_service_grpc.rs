use std::net::SocketAddr;
use std::sync::Arc;
use rocksdb::DB;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use example::vehicle_server::{Vehicle, VehicleServer};
use example::{GetLocationRequest, GetLocationResponse, VehicleLocation};
use crate::data_model::Location;
use log::info;

pub mod example {
    tonic::include_proto!("example");
}

#[derive(Debug)]
pub struct VehicleService {
    db: Arc<DB>,
}

#[tonic::async_trait]
impl Vehicle for VehicleService {
    async fn get_location(
        &self,
        request: Request<GetLocationRequest>,
    ) -> Result<Response<GetLocationResponse>, Status> {
        let id = &request.get_ref().vehicle_id;
        let response = match self.db.get(id) {
            Ok(Some(response)) => {
                match Location::from_bytes(response) {
                    Ok(location) => GetLocationResponse {
                        success: true,
                        message: "".to_string(),
                        vehicle_location: Some(VehicleLocation {
                            vehicle_id: id.to_string(),
                            longitude: location.get_lng(),
                            latitude: location.get_lat(),
                        }),
                    },
                    Err(e) => GetLocationResponse {
                        success: false,
                        message: e.message,
                        vehicle_location: None,
                    }
                }
            }
            Ok(None) => GetLocationResponse {
                success: false,
                message: format!("Vehicle {} not found", id),
                vehicle_location: None,
            },
            Err(e) => GetLocationResponse {
                success: false,
                message: e.to_string(),
                vehicle_location: None,
            }
        };
        Ok(Response::new(response))
    }
}

impl VehicleService {
    pub(crate) fn new(db: Arc<DB>) -> VehicleService {
        VehicleService { db }
    }

    pub(crate) async fn start(db: Arc<DB>, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting gRPC sever");
        let service = VehicleService::new(db);
        Server::builder()
            .add_service(VehicleServer::new(service))
            .serve(addr)
            .await?;
        Ok(())
    }
}

