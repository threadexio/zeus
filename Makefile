BUILD_TYPE ?= debug
CARGO_ARGS ?=

VERSION	?= $(shell git describe --tags --always)

PREFIX ?= /usr/local
DESTDIR ?=

ifeq ($(BUILD_TYPE),debug)
	CARGO_ARGS +=
else ifeq ($(BUILD_TYPE),release)
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
	-rm builder.tar.gz
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

.PHONY:
assets: FORCE
	scour \
		-i assets/logo.inkscape.svg \
		-o assets/logo.optimized.svg \
		--enable-id-stripping \
		--strip-xml-space \
		--no-line-breaks \
		--enable-comment-stripping \
		--shorten-ids \
		--remove-descriptive-elements \
		--create-groups 

	inkscape -C -w $(WIDTH) -h $(HEIGHT) \
		-o assets/logo.$(WIDTH)x$(HEIGHT).png \
		--export-type=png \
		assets/logo.inkscape.svg

.PHONY:
assets_clean: FORCE
	-rm assets/logo.optimized.svg
	-rm assets/*.png