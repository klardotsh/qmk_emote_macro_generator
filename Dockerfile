FROM rust:1.28.0-slim

WORKDIR /usr/src/myapp

COPY ./LICENSE ./
COPY ./README.md ./
COPY ./Cargo.toml ./
COPY ./Cargo.lock ./
COPY ./src ./src

RUN cargo install --path .

COPY config.toml ./

CMD ["qmk_emote_macro_generator"]
