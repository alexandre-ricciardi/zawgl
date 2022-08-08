FROM rust:latest as builder

COPY . .

RUN cargo build --release --quiet

FROM rust:slim-buster

COPY --from=builder target/release/zawgl /zawgl/
COPY --from=builder .config /zawgl/.config

EXPOSE 8182
WORKDIR /zawgl
CMD ["./zawgl"]