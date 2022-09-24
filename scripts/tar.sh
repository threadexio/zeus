#!/bin/bash
set -e

# Arguments:
# 1: DESTDIR
# 2: Out archive

DESTDIR="${1:-target/package}"
export DESTDIR

mkdir -p "$DESTDIR"

make \
	build completions install

tar -acvpf "${2:-zeus.tar.gz}" \
	-C "$DESTDIR/" \
	--owner=0 --group=0 \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- "."
