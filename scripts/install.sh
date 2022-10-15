#!/bin/bash
set -e

install() {
	command install -D "$@"
}

# $1 - Overlay src directory
# $2 - Destination directory
install_overlay() {
	local src="${1}"
	local dst="${2}"

	local i
	local k

	mapfile -t dirs < <(find "$src" -type d | sed -E "s|^${src}/?+||g")

	for i in "${dirs[@]}"; do
		k="$(eval echo "$i")"
		mkdir -p "$dst/$k"
	done

	mapfile -t files < <(find "$src" -type f | sed -E "s|^${src}/?+||g")

	for i in "${files[@]}"; do
		k="$(eval echo "$i")"
		install "$src/$i" "$dst/$k"
	done
}

[ ! -d "$DESTDIR/" ] && \
	echo "$DESTDIR: No such file or directory" && \
	exit 1

# Main overlay
install_overlay rootdir/ "$DESTDIR"

# Extra Directories
mkdir -p \
	"$DESTDIR/$DATA_DIR" \
	"$DESTDIR/$RUNTIME_DIR" \
	"$DESTDIR/$BUILD_DIR"

chmod 777 "$DESTDIR/$BUILD_DIR"

# Runtime overlays
for i in runtimes/*; do
	[ -d "$i/overlay" ] && install_overlay "$i/overlay" "$DESTDIR"
done

# Runtimes
for i in target/"$BUILD_PROFILE"/librt_*.so; do
	install -m 644 "./$i" -t "$DESTDIR/$PREFIX/lib/zeus/runtimes"
done

# zeus & builder
install -m 755 "target/$BUILD_PROFILE/zeus" -t "$DESTDIR/$PREFIX/bin"
install -m 755 "target/$BUILD_PROFILE/builder" -t "$DESTDIR/$PREFIX/share/zeus"
