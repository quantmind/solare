FROM quantmind-solana

ARG USER
RUN useradd -ms /bin/bash ${USER}

ENV USER ${USER}

USER ${USER}

WORKDIR /home/${USER}

COPY ./devops/images/local/bin/* /usr/local/bin/
