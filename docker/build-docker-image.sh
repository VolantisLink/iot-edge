#!/bin/bash

docker build -f Dockerfile.gnu -t rustembedded/cross-custom:armv5te-unknown-linux-gnueabi-0.2.1 .
docker build -f Dockerfile.musl -t rustembedded/cross-custom:armv5te-unknown-linux-musleabi-0.2.1 .