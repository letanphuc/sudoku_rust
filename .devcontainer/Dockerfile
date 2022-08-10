# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.231.5/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && apt-get -y upgrade && apt-get install -y fzf
User vscode
RUN echo "source /usr/share/doc/fzf/examples/completion.bash" >> ~/.bashrc
RUN echo "source /usr/share/doc/fzf/examples/key-bindings.bash" >> ~/.bashrc