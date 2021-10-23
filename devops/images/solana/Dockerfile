FROM --platform=$BUILDPLATFORM ubuntu:20.04
LABEL maintainer quantmind

ARG sol_version=v1.8.1

RUN sh -c "$(curl -sSfL https://release.solana.com/${sol_version}/install)"
