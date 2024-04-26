# Add Docker to the APT source list.
# https://docs.docker.com/engine/install/ubuntu/#install-using-the-repository
curl -fsSL https://download.docker.com/linux/ubuntu/gpg \
| gpg --dearmor -o /usr/local/share/keyrings/docker-apt-keyring.gpg

echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/usr/local/share/keyrings/docker-apt-keyring.gpg] \
    https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) stable" \
> /etc/apt/sources.list.d/docker.list

# Update APT with the new package source.
# - Dir::Etc::sourcelist specifies the source to update.
# - Dir::Etc::sourceparts="-" ignores other files in the source list directory.
# - APT::Get::List-Cleanup="false" tells APT to not clean up stale information.
apt-get update \
    -o Dir::Etc::sourcelist="/etc/apt/sources.list.d/docker.list" \
    -o Dir::Etc::sourceparts="-" \
    -o APT::Get::List-Cleanup="false"

apt-get install -y --no-install-recommends \
    containerd.io \
    docker-ce \
    docker-ce-cli \
    docker-buildx-plugin \
    docker-compose-plugin
