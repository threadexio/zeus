#!/usr/bin/bash
set -eu
shopt -s nullglob

REPO_DIR="$(dirname "$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )")"
PROFILE_DIR="${REPO_DIR}/profiles"
PROFILE_PATH="${PROFILE_DIR}/${PROFILE}.env"

function info {
	printf ' \e[0;36m[*]\e[0m %s\n' "$@"
}

function warn {
	printf ' \e[0;33m[!]\e[0m %s\n' "$@"
}

function load_profile {
	# auto export all variables
	set -a
	# shellcheck source=/dev/null
	. "${PROFILE_PATH}"
	set +a
}

DESTDIR="$(realpath -e "$DESTDIR")"
export DESTDIR

load_profile

# Main overlay
info "Installing main overlay"
info "======================="
turboinstall -p "${PROFILE_PATH}" -- "$DESTDIR" "${REPO_DIR}/overlay"

printf '\n'
info "Installing runtime overlays"
info "==========================="
turboinstall -p "${PROFILE_PATH}" -- "$DESTDIR" "${REPO_DIR}"/runtimes/*/overlay
