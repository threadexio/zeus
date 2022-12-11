#!/bin/bash
set -eux

mkdir -p "$DESTDIR"
DESTDIR="$(realpath -e "./build/pkgroot")"
export DESTDIR


./scripts/install.sh

tar -acvpf "${1:-zeus.tar.gz}" \
	-C "$DESTDIR" \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- .
