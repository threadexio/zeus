#!/bin/bash
set -eux

mkdir -p ./build/pkgroot

DESTDIR="$(realpath -e "./build/pkgroot")"
export DESTDIR

./scripts/install.sh

tar -acvpf "${1:?Output archive not set}" \
	-C "$DESTDIR" \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- .
