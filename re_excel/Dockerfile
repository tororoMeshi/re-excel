# ────────── Build Stage ──────────
FROM rust:1.86 AS builder
WORKDIR /app

# 依存解決レイヤー：Cargo.toml のみコピーして fetch
COPY Cargo.toml ./
RUN mkdir src     && echo 'fn main() { println!("hello"); }' > src/main.rs     && cargo fetch     && rm -rf src

COPY . .
RUN cargo build --release

# ────────── Runtime Stage ──────────
FROM gcr.io/distroless/base-nossl-debian12:nonroot

COPY --from=builder /app/target/release/re_excel /usr/local/bin/re_excel

ENTRYPOINT ["/usr/local/bin/re_excel"]
