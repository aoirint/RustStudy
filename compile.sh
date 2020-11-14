#!/bin/sh

docker run --rm -it \
    -u "${UID}:${GID}" \
    -w /work \
    -v "${PWD}:/work" \
    rust \
    rustc "$@"
