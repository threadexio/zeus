#!/bin/sh
set -e

USER="$(whoami)"
HOSTNAME="$(hostname)"

RUSTC="$(rustc --version)"

DATE="$(date -u +"%a %b %d %I:%M:%S %p %Z %Y")"

echo "($USER@$HOSTNAME) ($RUSTC) $DATE"
