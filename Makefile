BUILD_TYPE ?= debug
CARGO_ARGS ?=

VERSION	?= $(shell git describe --tags --always)

PREFIX ?= /usr/local
DESTDIR ?=

ifeq ($(BUILD_TYPE),release)
	CARGO_ARGS += --release
endif

.PHONY:
all: build

.PHONY:
FORCE: ; 

.PHONY:
.ONESHELL:
build: FORCE
	export VERSION="$(VERSION)"
	cargo build $(CARGO_ARGS) --

	tar -acvf builder.tar.gz \
		-C $$PWD/builder/               . \
		-C $$PWD/target/$(BUILD_TYPE)/  builder

.PHONY:
clean: FORCE
	-cargo clean $(CARGO_ARGS) --

.PHONY:
package: build FORCE
	tar -acvf zeus-bin.tar.gz \
		-C $$PWD/target/$(BUILD_TYPE)/  zeus \
		-C $$PWD/                       builder.tar.gz

.PHONY:
install: build
	install -Dm0755 -t "$(DESTDIR)/$(PREFIX)/bin" target/$(BUILD_TYPE)/zeus
	install -Dm0644 -t "$(DESTDIR)/$(PREFIX)/share/zeus" builder.tar.gz

	mkdir -p "$(DESTDIR)/var/cache/aur"
	chmod 0777 "$(DESTDIR)/var/cache/aur"