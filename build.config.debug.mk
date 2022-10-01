# cargo's build type (debug or release)
export BUILD_PROFILE := debug

# Defaults
export LOG_LEVEL	 := info
export RUNTIME		 := docker
export BUILDER_NAME	 := zeus-builder
export BUILDER_IMAGE := zeus-builder
export AUR_URL		 := https://aur.archlinux.org/
export PREFIX		 := /usr/local
export BUILD_DIR	 := /var/cache/aur
export RUNTIME_DIR	 := $(PREFIX)/lib/zeus/runtimes
export DATA_DIR		 := $(PREFIX)/share/zeus
export VERSION		 := $(shell scripts/version.sh)
export BUILD_INFO	 := $(shell scripts/build_info.sh)
