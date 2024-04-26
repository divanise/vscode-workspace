WORK_DIR="$(mktemp -d)"
pushd "${WORK_DIR}"

DENO_VERSION=1.42.4
DENO_ARCH=$(uname -m | sed -e 's/arm64/aarch64/' -e 's/amd64/x86_64/')

curl -fsSL https://github.com/denoland/deno/releases/download/v${DENO_VERSION}/deno-${DENO_ARCH}-unknown-linux-gnu.zip \
    --output deno.zip

unzip deno.zip
chmod 755 deno
mv deno /usr/bin/deno

popd
rm -rf "${WORK_DIR}"
unset WORK_DIR
