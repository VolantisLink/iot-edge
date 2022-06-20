#!/bin/bash

podman pull docker.io/rustembedded/cross:armv5te-unknown-linux-musleabi-0.2.1
buildah bud -f Dockerfile.musl -t rustembedded/cross-custom:armv5te-unknown-linux-musleabi-0.2.1 .
