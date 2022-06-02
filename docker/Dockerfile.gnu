FROM rustembedded/cross:armv5te-unknown-linux-gnueabi-0.2.1

RUN apt update \
    && apt install -y capnproto