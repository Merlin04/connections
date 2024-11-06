#FROM ghcr.io/getzola/zola:v0.17.1 as zola
FROM rust:slim-bullseye AS zola

RUN apt-get update -y && \
  apt-get install -y make g++ libssl-dev

WORKDIR /app
COPY ./zola .

RUN cargo build --release

FROM rust:1.82-bullseye as builder
WORKDIR /usr/src/connections
COPY ./mail-handler .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/connections/target/release/connections /usr/local/bin/connections
COPY --from=zola /app/target/release/zola /bin/zola
#COPY ./ssg /usr/src/ssg

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

CMD ["connections"]