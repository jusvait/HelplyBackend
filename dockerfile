FROM rust:1.65

RUN apt-get install -y libpq-dev

RUN USER=root cargo new --bin helply_backend
WORKDIR /helply_backend

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/helply_backend*
RUN cargo install --path .

CMD ["helply_backend"]