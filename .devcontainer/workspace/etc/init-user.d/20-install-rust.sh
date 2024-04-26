WORK_DIR="$(mktemp -d)"
pushd "${WORK_DIR}"

RUST_TOOLCHAIN=1.77.2
RUST_PROFILE=default

curl -sSfO https://static.rust-lang.org/rustup/dist/$(uname -m)-unknown-linux-gnu/rustup-init
chmod +x rustup-init

./rustup-init \
  --default-toolchain 1.77.2 \
  --profile default \
  --no-modify-path \
  -y

$HOME/.cargo/bin/rustup component add \
  clippy \
  rust-analyzer \
  rust-src \
  rustfmt

popd
rm -rf "${WORK_DIR}"
unset WORK_DIR
