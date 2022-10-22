#!/bin/bash
set -eu
# shellcheck source=common.sh
. scripts/common.sh

export DESTDIR="target/$BUILD_PROFILE/package"

mkdir -p "$DESTDIR"

./scripts/install.sh

tar -acvpf "${1:-zeus.tar.gz}" \
	-C "$DESTDIR" \
	--no-acls \
	--no-selinux \
	--no-xattrs \
	-- "."
