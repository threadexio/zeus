#!/bin/bash
set -eu
# shellcheck source=common.sh
. scripts/common.sh

install() {
	./scripts/helper.py install "$@"
}

install_bin() {
	install --strip --mode 755 "$1" "$2"
}

install_lib() {
	install --strip --mode 644 "$1" "$2"
}

install_file() {
	install --mode 644 "$1" "$2"
}

# $1 - Overlay src directory
# $2 - Destination directory
install_overlay() {
	./scripts/helper.py install_overlay "$1" "$2"
}

install_overlay rootdir/ "$DESTDIR"

# Runtime overlays
for i in runtimes/*; do
	if [ -d "$i/overlay" ]; then
		install_overlay "$i/overlay" "$DESTDIR"
	else
		warn "$i: No overlay found. Skipping..."
	fi
done

# Runtimes
for i in target/"$BUILD_PROFILE"/librt_*.so; do
	install_lib "$i" "$DESTDIR/$RUNTIME_DIR/${i##*/}"
done

# zeus & builder
install_bin "target/$BUILD_PROFILE/zeus" "$DESTDIR/$PREFIX/bin/zeus"
install_bin "target/$BUILD_PROFILE/builder" "$DESTDIR/$DATA_DIR/builder"
