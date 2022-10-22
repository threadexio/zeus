#!/bin/bash

set_if_empty() {
	[ -z "${!1}" ] && eval "$1"=\""$2"\"
}

set_if_empty USER "$(whoami 2>/dev/null)"
set_if_empty USER "$(id -n -u 2>/dev/null)"
set_if_empty USER "$(getent passwd "$(stat -c "%u" /proc/$$/)"|cut -d: -f1 2>/dev/null)"

set_if_empty HOSTNAME "$(hostname 2>/dev/null)"
set_if_empty HOSTNAME "$(uname -n 2>/dev/null)"
set_if_empty HOSTNAME "$(cat /proc/sys/kernel/hostname  2>/dev/null)"

set_if_empty RUSTC "$(rustc --version 2>/dev/null)"

set_if_empty DATE "$(date -u +'%a %b %d %I:%M:%S %p %Z %Y')"

set -u
echo "(${USER:-unknown}@${HOSTNAME:-unknown}) (${RUSTC:-rustc not found}) ${DATE:-date unknown}"
