FROM rust:bullseye AS chef
WORKDIR /newsletter
RUN cargo install cargo-chef --locked; rm -rf $CARGO_HOME/registry; apt update && apt install -y lld clang;

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path chef.recipe.json;

FROM chef AS builder
COPY --from=planner /newsletter/chef.recipe.json chef.recipe.json
RUN cargo chef cook --release --recipe-path chef.recipe.json;

ENV SQLX_OFFLINE=true
COPY . .
RUN cargo build --release --bin newsletter;

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y; \
    apt-get install -y --no-install-recommends openssl ca-certificates; \
    apt-get autoremove -y; \
    apt-get clean -y; \
    rm -rf /var/lib/apt/lists/*;

COPY --from=builder /newsletter/target/release/newsletter newsletter
COPY configuration configuration

ENV APP_ENV=production
ENTRYPOINT ["/app/newsletter"]

