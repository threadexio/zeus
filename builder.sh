#!/bin/bash
set -e

builddir="/build"
sudo chmod o+rw "$builddir"

bash

while IFS= read -r package; do
	cd $builddir || exit 1

	git clone "https://aur.archlinux.org/${package}.git" "${package}" &&
		cd "${package}" &&
		makepkg -s
done <"$builddir/.build"

sudo rm $builddir/.build
