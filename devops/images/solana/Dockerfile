FROM --platform=$BUILDPLATFORM ubuntu:20.04
LABEL maintainer quantmind

ARG sol_version=1.8.1

RUN apt update &&\
    apt install -y \
    curl

RUN sh -c "$(curl -sSfL https://release.solana.com/v${sol_version}/install)"
RUN mv /root/.local/share/solana/install/releases/${sol_version}/solana-release /solana &&\
    rm -rf /root/.local


ENV PATH="/solana/bin:$PATH"
