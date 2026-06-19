FROM rust:1.96-trixie AS builder
WORKDIR /home/fire-alarm-service
COPY examples/ examples/
COPY src/ src/
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --all-targets --release

FROM debian:trixie-slim
WORKDIR /home
ARG DEBIAN_FRONTEND=noninteractive
RUN apt update && apt full-upgrade --yes && apt install curl --yes && \
    curl --show-error --silent https://dotenvx.sh/install.sh | sh && \
    apt remove curl --yes && apt autoremove --yes && apt clean
COPY .env.test .
COPY index.html .
COPY timestamp.xmpl timestamp.txt
COPY --from=builder /home/fire-alarm-service/target/release/fire-alarm-service /usr/local/bin/
COPY --from=builder /home/fire-alarm-service/target/release/examples/wmata /usr/local/bin/
ENTRYPOINT [ "dotenvx", "run", "--" ]
CMD [ "fire-alarm-service" ]