FROM ubuntu:noble-20240605

ARG USERNAME=vscode
ARG USER_UID=1001
ARG USER_GID=${USER_UID}
ARG USER_SHELL_PACKAGE=zsh
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
# Add a non-root user.
#
RUN \
    #
    # Install necessary packages
    apt-get install -y --no-install-recommends sudo ${USER_SHELL_PACKAGE} && \
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
# Copy our utilities.
#
COPY bin/                   /usr/local/bin/

#
# Set up our environment variable directory.
#
RUN \
    mkdir -p /usr/local/etc/environment.d && \
    chown ${USERNAME}:${USERNAME} /usr/local/etc/environment.d

#
# Switch to running as the user.
#
USER ${USER_UID}

#
# Run the image initialization scripts, layering based
# on the scripts' initial digit.
#
COPY etc/init-image.d/0*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^0'  /usr/local/etc/init-image.d/

COPY etc/init-image.d/1*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^1'  /usr/local/etc/init-image.d/

COPY etc/init-image.d/2*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^2'  /usr/local/etc/init-image.d/

COPY etc/init-image.d/3*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^3'  /usr/local/etc/init-image.d/

COPY etc/init-image.d/4*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^4'  /usr/local/etc/init-image.d/

COPY etc/init-image.d/5*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^5'  /usr/local/etc/init-image.d/

COPY etc/init-image.d/6*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^6'  /usr/local/etc/init-image.d/

COPY etc/init-image.d/7*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^7'  /usr/local/etc/init-image.d/

COPY etc/init-image.d/8*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^8'  /usr/local/etc/init-image.d/

COPY etc/init-image.d/9*    /usr/local/etc/init-image.d/
RUN run-parts --regex '^9'  /usr/local/etc/init-image.d/

#
# Clean up APT.
#
RUN \
    sudo apt-get clean && \
    sudo rm -rf /var/lib/apt/lists

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
