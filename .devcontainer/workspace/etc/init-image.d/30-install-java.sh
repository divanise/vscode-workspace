# Based on https://github.com/adoptium/containers/blob/main/21/jdk/ubuntu/jammy/Dockerfile

WORK_DIR="$(mktemp -d)"
pushd "${WORK_DIR}"

JAVA_HOME=/opt/java/openjdk
JAVA_VERSION=21.0.3+9

case "$(uname -m)" in
    aarch64|arm64)
        JAVA_SHA256_SUM='7d3ab0e8eba95bd682cfda8041c6cb6fa21e09d0d9131316fd7c96c78969de31'
        JAVA_ARCH='aarch64'
        ;;
    x64|x86_64|amd64)
        JAVA_SHA256_SUM='fffa52c22d797b715a962e6c8d11ec7d79b90dd819b5bc51d62137ea4b22a340'
        JAVA_ARCH='x64'
        ;;
esac;

JAVA_BINARY_URL="https://github.com/adoptium/temurin21-binaries/releases/download"
JAVA_BINARY_URL="${JAVA_BINARY_URL}/jdk-$(echo ${JAVA_VERSION} | jq -Rr @uri)"
JAVA_BINARY_URL="${JAVA_BINARY_URL}/OpenJDK21U-jdk_${JAVA_ARCH}_linux_hotspot"
JAVA_BINARY_URL="${JAVA_BINARY_URL}_$(echo ${JAVA_VERSION} | sed -e 's/+/_/').tar.gz"

curl -fsSL ${JAVA_BINARY_URL} --output openjdk.tar.gz

echo "${JAVA_SHA256_SUM} *${WORK_DIR}/openjdk.tar.gz" | sha256sum -c -

mkdir -p "$JAVA_HOME"
tar --extract \
    --file ${WORK_DIR}/openjdk.tar.gz \
    --directory "${JAVA_HOME}" \
    --strip-components 1 \
    --no-same-owner \

popd
rm -rf "${WORK_DIR}"
unset WORK_DIR
