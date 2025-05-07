# -----------------------------------
FROM jkutkut/docker4rust as dev

RUN cargo install wasm-pack
# rust-std for wasm32-unknown-unknown
RUN rustup target add wasm32-unknown-unknown
# wasm-bindgen
RUN cargo install wasm-bindgen-cli

# -----------------------------------
FROM alpine:3.16 as release

# RUN apk add --update XXXXXXXX && \
#   rm -rf /var/cache/apk/*

WORKDIR /app

COPY target/release/XXXXXXXX/XXXXXXXX .

ENTRYPOINT ["./XXXXXXXX"]
