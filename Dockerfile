FROM rust:latest
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  fonts-inter \
  fonts-dejavu-core \
  fontconfig \
  && rm -rf /var/lib/apt/lists/* \
  && fc-cache -f -v
COPY Cargo.toml Cargo.lock ./
COPY assets ./assets
COPY src ./src
RUN cargo build --release --locked
ENV PORT=3000
ENV IP=0.0.0.0
EXPOSE 3000
CMD ["/app/target/release/links"]
