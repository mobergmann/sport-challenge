# syntax=docker/Dockerfile:1.2
FROM ubuntu:latest

LABEL org.opencontainers.image.source=https://github.com/mobergmann/sport-challenge

COPY ./sqlite/ /src/sqlite
COPY ./public/ /src/public
COPY ./target/release/sport-challenge /src/sport-challenge

WORKDIR /src

CMD "./sport-challenge"
