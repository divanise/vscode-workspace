WORK_DIR="$(mktemp -d)"
pushd "${WORK_DIR}"

MAVEN_HOME=/opt/java/maven
MAVEN_VERSION=3.9.6
MAVEN_SHA512_SUM=706f01b20dec0305a822ab614d51f32b07ee11d0218175e55450242e49d2156386483b506b3a4e8a03ac8611bae96395fd5eec15f50d3013d5deed6d1ee18224

curl -fsSL https://dlcdn.apache.org/maven/maven-3/${MAVEN_VERSION}/binaries/apache-maven-${MAVEN_VERSION}-bin.tar.gz \
    --output maven.tar.gz

echo "${MAVEN_SHA512_SUM} *${WORK_DIR}/maven.tar.gz" | sha512sum -c -

tar tzvf maven.tar.gz

mkdir -p "$MAVEN_HOME"
tar --extract \
    --file ${WORK_DIR}/maven.tar.gz \
    --directory "${MAVEN_HOME}" \
    --strip-components 1 \
    --no-same-owner \

popd
rm -rf "${WORK_DIR}"
unset WORK_DIR
