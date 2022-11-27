#!/bin/bash
set -eux

mkdir ./build/pkgroot
DESTDIR="$(realpath -e "./build/pkgroot")"

export DESTDIR

./scripts/install.sh

tar -acvpf "zeus.tar.gz" \
	-C "$DESTDIR" \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- .

rm -rf "${DESTDIR:?}"
