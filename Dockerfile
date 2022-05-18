FROM rust:1.60 AS builder

WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo new backend

WORKDIR /usr/src/backend
RUN cargo search --limit 0
COPY backend/Cargo.toml Cargo.lock ./
RUN cargo fetch
RUN cargo build --release --target x86_64-unknown-linux-musl

COPY ./backend/src ./src/
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM node AS fe-builder
COPY frontend/package.json frontend/package-lock.json ./
RUN npm install
COPY frontend .
RUN npm run build

FROM debian
RUN apt-get update && apt-get -y upgrade
RUN apt-get install -y python3-pip
RUN pip install brother_ql

COPY --from=fe-builder dist dist/
COPY --from=builder /usr/local/cargo/bin/backend .
CMD ["./backend"]
