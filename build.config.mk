export LOG_LEVEL     ?= debug
export RUNTIME       ?= docker
export BUILDER_NAME  ?= zeus-builder
export BUILDER_IMAGE ?= zeus-builder
export AUR_URL       ?= https://aur.archlinux.org/

export PREFIX        ?= /usr/local
export DATA_DIR      ?= $(PREFIX)/share/zeus
export LIB_DIR       ?= $(PREFIX)/lib/zeus
export RUNTIME_DIR   ?= $(LIB_DIR)/runtimes
export BUILD_DIR     ?= /var/cache/aur

export VERSION       ?= $(shell scripts/version.sh)
export BUILD_INFO    ?= $(shell scripts/build_info.sh)

# cargo's build type (debug or release)
export BUILD_PROFILE ?= debug
