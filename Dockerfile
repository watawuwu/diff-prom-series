FROM rust AS builder

WORKDIR /app

ADD Cargo.toml .
ADD Cargo.lock .

RUN mkdir src && \
    echo 'fn main(){}' >  src/main.rs && \
    cargo fetch

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/diff-prom-series /bin/diff-prom-series

ENTRYPOINT ["/bin/diff-prom-series"]

