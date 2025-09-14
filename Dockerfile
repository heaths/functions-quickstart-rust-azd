FROM mcr.microsoft.com/devcontainers/rust:1-bullseye AS build
WORKDIR /usr/src/functions-quickstart-rust-azd
COPY . .
RUN cargo build --release

FROM rust:1-slim-bullseye
COPY host.json /usr/local/bin/
COPY hello/function.json /usr/local/bin/hello/
COPY --from=build /usr/src/functions-quickstart-rust-azd/target/release/handler /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/handler"]
