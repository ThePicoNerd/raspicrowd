# Base image for rapsberrypi 3 target
FROM rustembedded/cross:arm-unknown-linux-gnueabi-0.2.1

RUN dpkg --add-architecture armel
RUN apt update
RUN apt install -y libdbus-1-dev pkg-config
