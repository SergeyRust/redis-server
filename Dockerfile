FROM rust:1.70

RUN USER=root cargo new --bin aichel_server
WORKDIR /aichel_server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/aichel_server*
RUN cargo install --path .
EXPOSE 8000

CMD ["aichel_server"]