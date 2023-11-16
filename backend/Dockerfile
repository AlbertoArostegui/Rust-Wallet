FROM rust:1.68 as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install libpq-dev -y && rm -rf /var/lib/apt/lists/*
ENV LD_LIBRARY_PATH=/path/to/library:$LD_LIBRARY_PATH
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
COPY .env .

CMD ["backend"]