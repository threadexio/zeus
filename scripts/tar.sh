#!/bin/bash
set -eux

DESTDIR="$(realpath -e "./build/pkgroot")"
mkdir -p "$DESTDIR"
export DESTDIR


./scripts/install.sh

tar -acvpf "${1:-zeus.tar.gz}" \
	-C "$DESTDIR" \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- .
