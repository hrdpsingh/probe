FROM docker.io/library/debian:latest
RUN apt-get update && apt-get install -y build-essential rustup
RUN rustup default stable
WORKDIR /app