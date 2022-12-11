#!/bin/bash
set -eux

if [ ! -d ./build/pkgroot ]; then
	mkdir ./build/pkgroot
fi

DESTDIR="$(realpath -e "./build/pkgroot")"
export DESTDIR


./scripts/install.sh

tar -acvpf "${1:-zeus.tar.gz}" \
	-C "$DESTDIR" \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- .
