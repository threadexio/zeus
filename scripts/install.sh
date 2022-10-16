#!/bin/bash
set -eu

cp() {
	command cp -L --preserve=mode --reflink=auto "$@"
}

# only for installing elf files
install() {
	command install -s -D "$@"
}

# $1 - Overlay src directory
# $2 - Destination directory
install_overlay() {
	local src="${1}"
	local dst="${2}"

	local i
	local k

	mapfile -t files < <(find -L "$src" -type f | sed -E "s|^${src}/?+||g")

	for i in "${files[@]}"; do
		k="$(eval echo "$i")"

		i="$src/$i"
		k="$dst/$k"

		mkdir -p "${k%/*}"

		if [ "${k##*/}" != ".gitkeep" ]; then
			cp "$i" "$k"
		fi
	done
}

[ ! -d "$DESTDIR/" ] && \
	echo "$DESTDIR: No such file or directory" && \
	exit 1

# Main overlay
install_overlay rootdir/ "$DESTDIR"

# Build directory

# TODO: Proper privilege separation

# this must match the uid & gid inside rootdir/usr/lib/sysusers.d/zeus.conf
chown -v 23248:23248 "$DESTDIR/$BUILD_DIR"
chmod -v 6770 "$DESTDIR/$BUILD_DIR"
# 6 -> all files "owned" by same group (directory setgid)
# 7 -> rwx  for owner
# 7 -> rwx for group
# 0 -> ---  for anyone else

# TODO: Check to see if we can somehow use acls in packaging, fakeroot doesnt support them
#setfacl -d -m user::rwx  "$DESTDIR/$BUILD_DIR"
#setfacl -d -m group::rwx "$DESTDIR/$BUILD_DIR"
#setfacl -d -m other::--- "$DESTDIR/$BUILD_DIR"

# Runtime overlays
for i in runtimes/*; do
	[ -d "$i/overlay" ] && install_overlay "$i/overlay" "$DESTDIR"
done

# Runtimes
for i in target/"$BUILD_PROFILE"/librt_*.so; do
	install -m 644 "./$i" -t "$DESTDIR/$RUNTIME_DIR"
done

# zeus & builder
install -m 755 "target/$BUILD_PROFILE/zeus" -t "$DESTDIR/$PREFIX/bin"
install -m 755 "target/$BUILD_PROFILE/builder" -t "$DESTDIR/$DATA_DIR"
