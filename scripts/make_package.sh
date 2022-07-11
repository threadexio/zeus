#!/bin/bash
set -ex

O="$PWD/zeus.tar.gz"

DESTDIR="$(mktemp -d -t zeus.XXXXXX)"
export DESTDIR

export BUILD_TYPE=release
export PREFIX=/usr

make build completions install

(cd "$DESTDIR" && tar -acvpf "$O" --owner=0 --group=0 -- *)
