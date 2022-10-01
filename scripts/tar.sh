#!/bin/sh
set -e

# Arguments:
# 1: Out archive

DESTDIR="target/$BUILD_PROFILE/package"

echo "$DESTDIR"

export DESTDIR

mkdir -p "$DESTDIR"

make \
	build install

tar -acvpf "${1:-zeus.tar.gz}" \
	-C "$DESTDIR/" \
	--owner=0 --group=0 \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- "."
