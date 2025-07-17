# syntax=docker/dockerfile:1.4
FROM ubuntu:noble-20240605

ARG USERNAME=vscode
ARG USER_UID=1001
ARG USER_GID=${USER_UID}

#
# Set environment variables.
#
ENV \
    LANG=C.UTF-8 \
    TZ=America/Toronto \
    DEBIAN_FRONTEND=noninteractive

#
# Make sure we're not ignoring errors.
#
SHELL ["/usr/bin/bash", "-e", "-o", "pipefail", "-c"]

#
# Initialize APT.
#
RUN <<EOT
    apt-get update
EOT

#
# Add a non-root user.
#
RUN <<EOT
    # Install necessary packages
    apt-get install -y --no-install-recommends sudo zsh

    # Create the user.
    groupadd --gid ${USER_GID} ${USERNAME}
    useradd --shell /usr/bin/zsh --uid ${USER_UID} --gid ${USER_GID} ${USERNAME}
    mkdir /home/${USERNAME}/{,workspace}
    chmod 0755 /home/${USERNAME}/{,workspace}
    chown ${USERNAME}:${USERNAME} /home/${USERNAME}/{,workspace}

    # Add the user as a sudoer.
    echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME}
    chmod 0440 /etc/sudoers.d/${USERNAME}
EOT

#
# Set up our environment variable directory.
#
RUN <<EOT
    mkdir -p /usr/local/etc/environment.d
    chown ${USERNAME}:${USERNAME} /usr/local/etc/environment.d
EOT

#
# Switch to running as the user.
#
USER ${USER_UID}

#
# Copy our utility directories.
#
COPY bin/                   /usr/local/bin/
COPY libexec/               /usr/local/libexec/

#
# Run the image initialization scripts.
#
COPY etc/init-image.d/      /usr/local/etc/init-image.d/
RUN run-parts               /usr/local/etc/init-image.d/

#
# Clean up APT.
#
RUN <<EOT
    sudo apt-get clean
    sudo rm -rf /var/lib/apt/lists
EOT

#
# Copy our home directory template.
#
COPY --chown=${USERNAME}:${USERNAME} home /home/${USERNAME}

#
# Copy the container initialization scripts.
#
COPY etc/post-create.d/     /usr/local/etc/post-create.d/
COPY etc/post-start.d/      /usr/local/etc/post-start.d/
COPY etc/post-attach.d/     /usr/local/etc/post-attach.d/

#
# Finally, our entrypoint.
#
CMD /bin/sh -c "while sleep 60; do :; done"
