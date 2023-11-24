FROM rust:1.70 as builder
LABEL repository="rust-kafka-sasl"

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye

RUN apt-get update \
    && apt-get install -y ca-certificates libssl-dev libsasl2-2 libsasl2-dev libsasl2-modules \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-kafka-sasl .

CMD ["./rust-kafka-sasl"]