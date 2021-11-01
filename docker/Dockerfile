FROM rust:latest as builder

COPY . .

RUN cargo build --release --quiet

FROM rust:slim-buster

COPY --from=builder target/release/og /og/
COPY --from=builder .config /og/.config

EXPOSE 8182
WORKDIR /og
CMD ["./og"]