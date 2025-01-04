FROM rust:latest

# VOLUME /newsletter
WORKDIR /newsletter

RUN apt update && apt install -y lld clang

ENV SQLX_OFFLINE=true
ENV APP_ENV=production

COPY . .

RUN cargo build --release

# ENTRYPOINT ["cargo", "run", "--release"]
ENTRYPOINT ["./target/release/newsletter"]

