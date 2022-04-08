FROM quantmind-solare

ARG USER
RUN useradd -ms /bin/bash ${USER}

ENV USER ${USER}

USER ${USER}

WORKDIR /solare

COPY ./devops/images/local/bin/* /usr/local/bin/
