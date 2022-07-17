FROM rust as project

RUN rustup component add rustfmt

ADD . /project
WORKDIR /project

COPY . .
RUN cargo fmt --all && \
    cargo build --workspace --release --verbose --color always && \
    cargo test --all-features --verbose --color always

FROM photon as app

ENV MODE="production" \
    PORT=8080 \
    RUST_LOG="info"

COPY --from=project /project/release/acme /acme

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp

ENTRYPOINT ["./acme"]