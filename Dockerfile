ARG VARIANT="nightly-bookworm-slim"

FROM rustlang/rust:${VARIANT}

ENV DEBIAN_FRONTEND noninteractive

RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive

RUN apt-get -qq install build-essential

RUN cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli

WORKDIR /home/clay/grokterm

COPY . .

CMD ["dx", "serve"]