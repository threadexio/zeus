#!/bin/bash

# this must match the uid & gid inside rootdir/usr/lib/sysusers.d/zeus.conf
chown -v 23248:23248 "./$BUILD_DIR"
chmod -v 6770 "./$BUILD_DIR"
# 6 -> all files "owned" by same group (directory setgid)
# 7 -> rwx  for owner
# 7 -> rwx for group
# 0 -> ---  for anyone else
