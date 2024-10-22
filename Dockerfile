#构建阶段
FROM rust:1.82.0
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release
#运行时阶段
# FROM rust:1.82.0-slim AS runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/chapter5 chapter5
COPY configuration configuration
ENV APP_ENVIRONMENT=production
# ENTRYPOINT [ "./target/release/chapter5" ]
ENTRYPOINT [ "./chapter5" ]