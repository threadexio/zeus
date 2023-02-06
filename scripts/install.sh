#!/bin/bash
set -eu

PROFILE_PATH="${PROFILE_PATH:?Profile path not set}"

# auto export all variables
set -a
# shellcheck source=/dev/null
. "${PROFILE_PATH}"
set +a

# install main overlay
turboinstall -p "${PROFILE_PATH}" -- "$DESTDIR" "overlay/"

# install runtime overlays
find runtimes/ \
	-maxdepth 2 -type d -name overlay \
	-exec turboinstall -p "${PROFILE_PATH}" -- "$DESTDIR" {} \;
