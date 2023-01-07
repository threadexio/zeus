#!/usr/bin/bash
set -eu

dst="$2"

# this must match the uid & gid inside overlay/usr/lib/sysusers.d/zeus.conf
chown -v 23248:23248 "$dst/$BUILD_DIR"
chmod -v 6770 "$dst/$BUILD_DIR"
# 6 -> all files "owned" by same group (directory setgid)
# 7 -> rwx  for owner
# 7 -> rwx  for group
# 0 -> ---  for anyone else

