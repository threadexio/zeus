#!/bin/bash
set -ex

O="$PWD/zeus.tar.gz"

DESTDIR="$(mktemp -d -t zeus.XXXXXX)"
export DESTDIR

export BUILD_TYPE=release
export PREFIX=/usr

make build completions
sudo -E make install

(cd "$DESTDIR" && tar -acvpf "$O" -- *)
