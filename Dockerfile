FROM rust:alpine as builder
RUN apk add --update --no-cache build-base musl-dev libc-dev openssl-dev binaryen curl node npm just
RUN rustup target add wasm32-unknown-unknown
RUN cargo install -f wasm-bindgen-cli
RUN npm install --global esbuild
WORKDIR /app
COPY . .
RUN just release

FROM alpine:latest
RUN apk add --update --no-cache python3
WORKDIR /app
COPY --from=builder /app/out /app
COPY --from=builder /app/server.py /app
CMD ["python3", "/app/server.py"]
