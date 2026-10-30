FROM rust:1-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/insp_backend /usr/local/bin/insp_backend
ENV DATABASE_URL=sqlite:///data/insp.db CONTENT_DIR=/content BIND_ADDR=0.0.0.0:8080
CMD ["insp_backend"]
