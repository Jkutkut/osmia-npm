# -----------------------------------
FROM jkutkut/docker4rust as dev

RUN cargo install wasm-pack
# rust-std for wasm32-unknown-unknown
RUN rustup target add wasm32-unknown-unknown
# wasm-bindgen
RUN cargo install wasm-bindgen-cli

# -----------------------------------
