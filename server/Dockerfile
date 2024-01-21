# Build Stage
FROM rust:latest AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new eit
WORKDIR /usr/src/eit
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM scratch AS bundle
COPY --from=builder /usr/local/cargo/bin/eit .
USER 1000

EXPOSE 8080
CMD ["./eit", "-p", "8080"]
