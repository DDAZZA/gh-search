FROM rust:1.46 as builder
WORKDIR /usr/src/

RUN USER=root cargo new --bin myapp
WORKDIR /usr/src/myapp

COPY Cargo.toml .
# COPY Cargo.lock .
RUN cargo build --release
RUN rm src/*.rs

RUN cargo clean
COPY ./src src
RUN cargo install --path .


FROM gcr.io/distroless/cc

VOLUME /workspace
WORKDIR workspace
USER 1000
ENTRYPOINT ["gh-search"]
COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/gh-search
