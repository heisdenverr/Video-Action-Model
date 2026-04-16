FROM rust:1.86 as build
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release || true
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/trust-box-backend /usr/local/bin/trust-box-backend
EXPOSE 8080
CMD ["trust-box-backend"]
