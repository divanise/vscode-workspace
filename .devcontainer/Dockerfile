FROM ubuntu:noble-20240423

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
# Run the image initialization scripts.
#
COPY workspace/bin/run-scripts      /usr/local/bin
COPY workspace/etc/init-image.d/    /usr/local/etc/init-image.d/

RUN source /usr/local/bin/run-scripts /usr/local/etc/init-image.d

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

COPY --chown=${USERNAME}:${USERNAME} workspace/home /home/${USERNAME}

#
# Run the remainder of the setup as the user.
#
USER ${USER_UID}

#
# Run the user initialization scripts.
#
COPY workspace/etc/init-user.d/     /usr/local/etc/init-user.d/

RUN source /usr/local/bin/run-scripts /usr/local/etc/init-user.d

#
# Copy the container initialization scripts.
#
COPY workspace/etc/post-create.d/   /usr/local/etc/post-create.d/
COPY workspace/etc/post-start.d/    /usr/local/etc/post-start.d/
COPY workspace/etc/post-attach.d/   /usr/local/etc/post-attach.d/

#
# Copy our utilities.
#
COPY workspace/bin/                 /usr/local/bin

#
# Finally, our entrypoint.
#
CMD /bin/sh -c "while sleep 60; do :; done"
