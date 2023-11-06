FROM ubuntu:latest

LABEL org.opencontainers.image.source=https://github.com/mobergmann/sport-challenge

# create base folder
RUN mkdir /src

# copy over all files neccesary for the execution of the program
COPY ./target/release/sport-challenge /src/sport-challenge
COPY ./public/ /src/public
# create volume and a symbolic link to the database folder
VOLUME /data
RUN mkdir /data
RUN touch /data/data.db
RUN ln -s /data/data.db /src/data.db

WORKDIR /src
# make the program executalbe
RUN chmod 555 sport-challenge
CMD "./sport-challenge"
