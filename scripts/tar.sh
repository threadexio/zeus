#!/bin/sh
set -eu

# Arguments:
# 1: Out archive

DESTDIR="target/$BUILD_PROFILE/package"
export DESTDIR

mkdir -p "$DESTDIR"

./scripts/install.sh

tar -acvpf "${1:-zeus.tar.gz}" \
	-C "$DESTDIR/" \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- "."
