FROM solare

ARG USER
RUN useradd -ms /bin/bash ${USER}
RUN chown -R ${USER}:${USER} /solana
RUN chown -R ${USER}:${USER} /solare
ENV USER ${USER}

USER ${USER}

# build so that bpf tools are installed
COPY Cargo.toml Cargo.lock Anchor.toml ./
COPY programs programs
RUN anchor build
