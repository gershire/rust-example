# Build the project within a linux container
FROM rust:slim-buster as builder

# Install project dependencies
RUN apt-get update -y && \
    apt-get install -y build-essential make git libgflags-dev libsnappy-dev cmake libclang-dev libtbb-dev zlib1g-dev libbz2-dev libjemalloc-dev

# Install RocksDB dependencies
RUN git clone https://github.com/facebook/rocksdb.git && \
    cd rocksdb && \
    DEBUG_LEVEL=1 make static_lib && \
    make install-static && \
    export LD_LIBRARY_PATH=/usr/local/lib && \
    ldconfig && \
    cd ../ && \
    rm -rf rocksdb && \
    cp /usr/lib/gcc/x86_64-linux-gnu/8/include/stddef.h /usr/lib/gcc/x86_64-linux-gnu/8/include/stdarg.h /usr/local/include/

WORKDIR /usr/src/example

# Copy the project into the container
COPY . .

# Update rustup to support edition 2021
# and build the executable
RUN rustup component add  rustfmt && \
    rustup toolchain install nightly --component rustfmt --component clippy --allow-downgrade && \
    rustup default nightly && \
    cargo build --release

# Build the working image
FROM debian
WORKDIR /usr/src/example

# Copy executable built on the previous stage
COPY --from=builder /usr/src/example/target/release/* /usr/src/example/

CMD ["./rust-rocksdb-example"]
