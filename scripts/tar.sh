#!/bin/bash
set -eux

if [ -z "$DESTDIR" ]; then
	DESTDIR="$(realpath "./build/pkgroot")"
fi

export DESTDIR

mkdir -p "$DESTDIR"

./scripts/install.sh

tar -acvpf "${1:-zeus.tar.gz}" \
	-C "$DESTDIR" \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- .
