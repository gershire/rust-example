FROM rust:slim-buster as builder

RUN apt-get update -y && \
    apt-get install -y build-essential make git libgflags-dev libsnappy-dev cmake libclang-dev libtbb-dev zlib1g-dev libbz2-dev libjemalloc-dev

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
COPY . .
RUN cargo build

FROM debian
WORKDIR /usr/src/example
COPY --from=builder /usr/src/example/target/debug/rust-rocksdb-example /usr/src/example/
CMD ["bash"]