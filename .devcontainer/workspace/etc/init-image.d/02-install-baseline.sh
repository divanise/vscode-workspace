# Install utilities.
apt-get install -y --no-install-recommends \
    bind9-host \
    bzip2 \
    dirmngr \
    dnsutils \
    file \
    gettext-base \
    gpg-agent \
    htop \
    iproute2 \
    iputils-ping \
    jq \
    less \
    locales \
    lsof \
    net-tools \
    netcat-openbsd \
    openssl \
    procps \
    psmisc \
    socklog \
    strace \
    sudo \
    tcpdump \
    tree \
    tzdata \
    unzip \
    uuid \
    vim \
    xz-utils \
    zsh

# Install basic development tools.
apt-get install -y --no-install-recommends \
    build-essential \
    gdb \
    git \
    libtool \
    lldb \
    nasm \
    pkg-config
