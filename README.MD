# Rust Example
This application demonstrates the following features implemented in **Rust**:
* gRPC server
* local RocksDB storage
* asynchronous Kafka consumer

##Table of contents
1. [Example Description](#example-description)
2. [Configuration](#configuration)
3. [Build and Run](#build-and-run)
    1. [From Source](#from-source)
    2. [Docker](#docker)
    3. [Docker Compose](#docker-compose)
4. [Using an Application](#using-an-application)

## Example Description
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

## Configuration
The application is configured by a `.yml` file. By default, it's `resources/app-conf.yml`,
but the path to the configuration file can be overridden by `CONFIG_PATH` environment variable.

|Configuration Property|Description|
|----------------------|-----------|
|database.path|Path to the RocksDB storage|
|grpc.socket_address|An address gRPC server will be binded to|
|kafka.bootstrap_server|Kafka cluster address|
|kafka.topic|Kafka topic to read messages from|

## Build and Run
There are several options on how to build and run the application. All require a Kafka cluster to
be accessible.

### From Source
You can run the application from source using `cargo run` command, or build the executable
by `cargo buld` and then run it from `target/debug/`. It requires [**cargo**](https://www.rust-lang.org/tools/install)
to be installed on your computer.

### Docker
The Dockerfile is included into the project. So you can build a docker image by the command
`docker build -t rust-example .`. This will take some time, because it builds some required
dependencies from source.

When you have the image built, you can run it with `docker run rust-example`.

### Docker Compose
If you don't have access to any Kafka cluster, you can run the application along with Kafka
with docker-compose. Of course, you need docker-compose to be installed on your computer.

Execute `docker-compose up -d` to run all you need out of the box.

## Using an Application
To create some data in the application, push messages into the Kafka topic with the following JSON format:
```
{
  "id": string,
  "location": {
    "lat": float,
    "lng": float
  }
}
```

To get data from the application, with any gRPC tool call `Vehicle.GetLocation()` function.
