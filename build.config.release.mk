# Overrides
export LOG_LEVEL     := info

export PREFIX        := /usr
export DATA_DIR      := $(PREFIX)/share/zeus
export LIB_DIR       := $(PREFIX)/lib/zeus

# cargo's build type (debug or release)
export BUILD_PROFILE := release
CARGO_ARGS += --release
