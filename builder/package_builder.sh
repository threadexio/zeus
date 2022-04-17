#!/bin/bash
set -e

# $1 - package name
# $2 - operation
# $.. - extra arguments for makepkg

if [ "$#" -lt 2 ]; then
	exit 1
fi

if [ ! -d "./$1" ]; then
	git clone "https://aur.archlinux.org/$1.git"
fi

cd "$1"

case "$2" in
"Build") ;;

"Upgrade")
	git pull
	;;
esac

shift
shift

makepkg -s --needed --noconfirm --noprogressbar "$@"
