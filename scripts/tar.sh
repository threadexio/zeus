#!/bin/bash
set -eux

O="${O:?Output dir not set}"
TARBALL="${TARBALL:?Output tarball path not set}"

WORKDIR="${O}/tar-pkg"
DESTDIR="${WORKDIR}/pkg"
export DESTDIR

mkdir -p \
	"$WORKDIR" \
	"$DESTDIR"

make install
tar -acvpf "$TARBALL" \
	-C "$DESTDIR" \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- .
