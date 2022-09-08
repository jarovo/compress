# Source: https://www.docker.com/blog/simplify-your-deployments-using-the-rust-official-image/

FROM rust as builder
WORKDIR /src/compressor
COPY . .
RUN cargo install --path . --color always

FROM ubuntu as runtime
COPY --from=builder /usr/local/cargo/bin/compressor /usr/local/bin/compressor
CMD ["compressor"]
