# Rust Example
This is an example project I have created to play with Rust.

Let's suppose we develop a microservice based vehicle tracking system.
Each vehicle has a unique ID registered in the system. The vehicles transmit their location
to some endpoint that passes the vehicle ID along with its location to a Kafka queue.

This project will represent a microservice that keeps the current geolocation
of each vehicle in the system and provides an API to other microservices
to get a vehicle location.

For that purpose, the microservice:
* consumes the **Kafka** topic
* stores the current locations of the vehicles in **RocksDB** database
* provides **gRPC** API to get the location of the vehicle by ID

## Docker
To build a docker image, run
```
docker build -t rust-example .
```

## TODO
* Move CLI to the separate bin
* Document
* Logging
* Config
* Unit tests