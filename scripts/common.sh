# shellcheck shell=bash

info() {
	printf "\e[0;36m => \e[0m %s\n" "$*"
}

warn() {
	printf "\e[0;33m !! \e[0m %s\n" "$*"
}

error() {
	printf "\e[0;31m => \e[0m %s\n" "$*"
}
