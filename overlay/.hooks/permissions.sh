#!/usr/bin/bash
set -eu

# from install.sh
export destdir

chmod -v 755 "$destdir/$PREFIX/bin/zeus"
chmod -v 755 "$destdir/$PREFIX/share/zeus/builder"

# this must match the uid & gid inside overlay/usr/lib/sysusers.d/zeus.conf
chown -v 23248:23248 "$destdir/$BUILD_DIR"
chmod -v 6770 "$destdir/$BUILD_DIR"
# 6 -> all files "owned" by same group (directory setgid)
# 7 -> rwx  for owner
# 7 -> rwx  for group
# 0 -> ---  for anyone else

