#!/usr/bin/bash
set -eu

dst="${2:?}"

# set by xtask
BUILD_ROOT="${BUILD_ROOT:?}"

install -vDm755 "$BUILD_ROOT/zeus" "$dst/$PREFIX/bin/zeus"
install -vDm755 "$BUILD_ROOT/builder" "$dst/$PREFIX/share/zeus/builder"

for rt in "$BUILD_ROOT"/librt_*.so; do
	install -vm644 "$rt" "$dst/$PREFIX/lib/zeus/runtimes"
done
