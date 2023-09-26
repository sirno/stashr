#!/usr/bin/env bash
#
# test.sh

docker volume create stashr-vdrive

docker build -t stashr .
docker run -it --rm -v stashr-vdrive:/mount/vdrive --name stashr-run stashr

docker volume rm stashr-vdrive

