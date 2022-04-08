FROM debian:bullseye-slim
LABEL maintainer quantmind

ARG sol_version=1.10.6

COPY ./devops/images/solare/install install

#RUN ./install/apt
#RUN ./install/rust


#RUN sh -c "$(curl -sSfL https://release.solana.com/v${sol_version}/install)"
#RUN mv /root/.local/share/solana/install/releases/${sol_version}/solana-release /solana &&\
#    rm -rf /root/.local


#ENV PATH="/solana/bin:$PATH"


#COPY ./devops/images/solare/install-node install-node
#RUN ./install-node && rm ./install-node
#RUN npm i -g yarn && npm i -g @project-serum/anchor-cli
