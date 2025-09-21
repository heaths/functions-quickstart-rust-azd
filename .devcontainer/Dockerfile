# Azure Functions CLI is only available for amd64 currently
FROM mcr.microsoft.com/devcontainers/javascript-node:22-bullseye
USER root
ENV CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:/usr/local/share/nvm/current/bin:/usr/local/share/npm-global/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
    RUSTUP_HOME=/usr/local/rustup
RUN npm install -g --no-audit --no-fund azure-functions-core-tools@4
RUN <<EOF
set -eux
ARCH=aarch64-unknown-linux-gnu
curl --proto '=https' --tlsv1.2 --silent --show-error --fail --location "https://static.rust-lang.org/rustup/dist/${ARCH}/rustup-init" --output rustup-init
chmod +x rustup-init
./rustup-init -y --no-modify-path --profile minimal --default-host "${ARCH}"
rm rustup-init
chmod -R a+w $RUSTUP_HOME $CARGO_HOME
rustup --version
rustc --version
cargo --version
EOF
ENTRYPOINT [ "/bin/sh" ]