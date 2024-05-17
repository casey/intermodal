FROM docker.io/library/rust:1.78.0-bookworm AS builder

WORKDIR /usr/local/src

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder /usr/local/src/target/release/imdl /usr/local/bin/imdl

ENTRYPOINT ["/usr/local/bin/imdl"]
