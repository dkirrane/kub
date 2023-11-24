ARG RUST_VERSION=1.74.0
ARG APP_NAME=kub

#
#
# todo - https://github.com/fede1024/rust-rdkafka/discussions/632
#
#

FROM rust:${RUST_VERSION} as builder

# Install necessary dependencies
RUN #apt-get update && apt-get install -y cmake

# Set the working directory
WORKDIR /usr/src

# Copy local files to the Docker image
COPY . .

# Compile the Rust application
RUN cargo build --release

# Start from a lightweight base image
FROM debian:buster-slim

## Install necessary dependencies
#RUN apt-get update \
#    && apt-get install -y \
#          ca-certificates tzdata \
#          openssl libssl-dev \
#          pkg-config \
#    && rm -rf /var/lib/apt/lists/*

RUN apt-get update \
    && apt-get install -y ca-certificates openssl libssl-dev libsasl2-2 libsasl2-dev libsasl2-modules \
    && rm -rf /var/lib/apt/lists/*

# # https://github.com/fede1024/rust-rdkafka/blob/master/Dockerfile#L3C1-L9C15
#RUN apt-get update && apt-get install -y build-essential \
#    curl \
#    openssl libssl-dev \
#    pkg-config \

# Copy the compiled Rust application from the builder image
COPY --from=builder /usr/src/target/release/${APP_NAME} /usr/local/bin

# Set the command to run your Rust application
CMD ["${APP_NAME}"]