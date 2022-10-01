#!/bin/sh
set -e

cp() {
	command cp --preserve=mode -r --reflink=auto "$@"
}

install() {
	command install -s -D "$@"
}

[ ! -d "$DESTDIR/" ] && \
	echo "$DESTDIR: No such file or directory" && \
	exit 1

# Extra files
cp rootdir/. "$DESTDIR/"

# Extra Directories
mkdir -p \
	"$DESTDIR/$DATA_DIR" \
	"$DESTDIR/$RUNTIME_DIR" \
	"$DESTDIR/$BUILD_DIR"

chmod 777 "$DESTDIR/$BUILD_DIR"

# Runtime data
for i in runtimes/*/data; do
	cp "./$i/." "$DESTDIR/$PREFIX/share/zeus"
done

# Runtimes
for i in target/"$BUILD_PROFILE"/librt_*.so; do
	install -m 644 "./$i" -t "$DESTDIR/$PREFIX/lib/zeus/runtimes"
done

# zeus & builder
install -m 755 "target/$BUILD_PROFILE/zeus" -t "$DESTDIR/$PREFIX/bin"
install -m 755 "target/$BUILD_PROFILE/builder" -t "$DESTDIR/$PREFIX/share/zeus"
