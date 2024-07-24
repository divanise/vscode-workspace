FROM ubuntu:noble-20240530

ARG USERNAME=vscode
ARG USER_UID=1001
ARG USER_GID=${USER_UID}
ARG USER_SHELL=/usr/bin/zsh

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
RUN apt-get update

#
# Run the image initialization scripts.
#
COPY etc/init-image.d/    /usr/local/etc/init-image.d/

RUN run-parts /usr/local/etc/init-image.d

#
# Add a non-root user.
#
RUN \
    #
    # Create the user.
    groupadd --gid ${USER_GID} ${USERNAME} && \
    useradd --shell ${USER_SHELL} --uid ${USER_UID} --gid ${USER_GID} ${USERNAME} && \
    mkdir /home/${USERNAME}/{,workspace} && \
    chmod 0755 /home/${USERNAME}/{,workspace} && \
    chown ${USERNAME}:${USERNAME} /home/${USERNAME}/{,workspace} && \
    #
    # Add the user as a sudoer.
    echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} && \
    chmod 0440 /etc/sudoers.d/${USERNAME}

COPY --chown=${USERNAME}:${USERNAME} home /home/${USERNAME}

#
# Run the remainder of the setup as the user.
#
USER ${USER_UID}

#
# Run the user initialization scripts.
#
COPY etc/init-user.d/     /usr/local/etc/init-user.d/

RUN run-parts /usr/local/etc/init-user.d

#
# Clean up APT.
#
RUN \
    sudo apt-get clean && \
    sudo rm -rf /var/lib/apt/lists

#
# Copy the container initialization scripts.
#
COPY etc/post-create.d/   /usr/local/etc/post-create.d/
COPY etc/post-start.d/    /usr/local/etc/post-start.d/
COPY etc/post-attach.d/   /usr/local/etc/post-attach.d/

#
# Copy our utilities.
#
COPY bin/                 /usr/local/bin

#
# Finally, our entrypoint.
#
CMD /bin/sh -c "while sleep 60; do :; done"
