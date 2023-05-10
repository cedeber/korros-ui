FROM rust:1.69-alpine as builder
RUN apk add --update --no-cache build-base musl-dev libc-dev openssl-dev binaryen curl nodejs npm just go
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli
RUN npm install --global esbuild lightningcss-cli
WORKDIR /app
COPY . .
RUN just release

FROM alpine:latest
# RUN apk add --update --no-cache python3
WORKDIR /app
COPY --from=builder /app/out /app
# COPY --from=builder /app/server.py /app
# CMD ["python3", "/app/server.py"]
COPY --from=builder /app/server /app
CMD ["server"]
