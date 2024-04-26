sudo chown :docker /var/run/docker.sock
sudo chmod ug=rw,o= /var/run/docker.sock

docker buildx create \
    --name multiarch \
    --node multiarch \
    --driver docker-container \
    > /dev/null
