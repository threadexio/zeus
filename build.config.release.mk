export BUILD_PROFILE := release

CARGO_ARGS += --release

export PREFIX := /usr

export BUILDER_NAME := zeus-builder
export BUILDER_IMAGE := zeus-builder
export BUILD_DIR := /var/cache/aur
export AUR_URL := https://aur.archlinux.org/
export RUNTIME := docker
export RUNTIME_DIR := $(PREFIX)/lib/zeus/runtimes
export DATA_DIR := $(PREFIX)/share/zeus
export LOG_LEVEL := info

export VERSION	:= $(shell scripts/version.sh)
export BUILD_INFO := $(shell scripts/build_info.sh)
