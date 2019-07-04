FROM rust as build

RUN apt-get update && apt-get -y install musl-tools && rm -rf /var/lib/apt/lists/*

COPY ./ ./

RUN rustup target install x86_64-unknown-linux-musl
#RUN CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

RUN mkdir -p /build-out

RUN cp target/x86_64-unknown-linux-musl/release/rust-gotham /build-out/

# Ubuntu 18.04
FROM scratch

#ENV DEBIAN_FRONTEND=noninteractive
#RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
#RUN apt-get update && apt-get -y install curl && rm -rf /var/lib/apt/lists/*
COPY --from=build /build-out/rust-gotham /rust

EXPOSE 80
ENTRYPOINT ["/rust"]