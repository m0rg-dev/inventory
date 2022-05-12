FROM rust AS builder

WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo new backend

WORKDIR /usr/src/backend
COPY backend/Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY backend/src ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY .env .
COPY --from=builder /usr/local/cargo/bin/backend .
USER 1000
CMD ["./backend"]
