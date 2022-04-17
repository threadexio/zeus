BUILD_TYPE ?= debug

VERSION	?= $(shell git rev-parse --short HEAD)

CARGO_ARGS ?=

ifeq ($(BUILD_TYPE),release)
	CARGO_ARGS += --release
endif

.PHONY:
all: build

.PHONY:
build:
	VERSION="$(VERSION)" cargo build $(CARGO_ARGS) --
	tar -acvf builder.tar.gz Dockerfile package_builder.sh -C target/$(BUILD_TYPE) builder

.PHONY:
clean:
	-rm builder.tar.gz
	-docker rm zeus-builder
	-docker rmi zeus-builder
	-cargo clean $(CARGO_ARGS) --

.PHONY:
package: build
	tar -acvf zeus-bin.tar.gz builder.tar.gz zeus
