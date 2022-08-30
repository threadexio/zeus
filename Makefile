BUILD_TYPE ?= debug
CARGO_ARGS ?=

VERSION	?= $(shell git describe --tags --always --dirty --broken)

PREFIX ?= /usr/local
DESTDIR ?=

DEFAULT_NAME ?= zeus-builder
DEFAULT_IMAGE ?= zeus-builder
DEFAULT_BUILDDIR ?= /var/cache/aur
DEFAULT_AUR_HOST ?= aur.archlinux.org
DEFAULT_RUNTIME ?= docker
DEFAULT_RUNTIME_DIR ?= $(PREFIX)/lib/zeus/runtimes
DEFAULT_DATA_DIR ?= $(PREFIX)/share/zeus

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
	export DEFAULT_NAME="$(DEFAULT_NAME)"
	export DEFAULT_IMAGE="$(DEFAULT_IMAGE)"
	export DEFAULT_BUILDDIR="$(DEFAULT_BUILDDIR)"
	export DEFAULT_AUR_HOST="$(DEFAULT_AUR_HOST)"
	export DEFAULT_RUNTIME="$(DEFAULT_RUNTIME)"
	export DEFAULT_RUNTIME_DIR="$(DEFAULT_RUNTIME_DIR)"
	export DEFAULT_DATA_DIR="$(DEFAULT_DATA_DIR)"

	export VERSION="$(VERSION)"
	cargo build --workspace $(CARGO_ARGS) --

.PHONY:
clean: FORCE
	-cargo clean $(CARGO_ARGS) --

.PHONY:
completions: FORCE
	./target/$(BUILD_TYPE)/zeus completions --shell bash > completions/zeus.bash
	./target/$(BUILD_TYPE)/zeus completions --shell zsh > completions/zeus.zsh
	./target/$(BUILD_TYPE)/zeus completions --shell fish > completions/zeus.fish

.PHONY:
install:
	install -Dm0755 -t "$(DESTDIR)/$(PREFIX)/bin" target/$(BUILD_TYPE)/zeus

	mkdir -p "$(DESTDIR)/var/cache/aur"
	chmod 0777 "$(DESTDIR)/var/cache/aur"

	install -Dm644 -t "$(DESTDIR)/etc/apparmor.d" apparmor/zeus

	mkdir -p "$(DESTDIR)/etc/apparmor.d/zeus.d"
	for i in apparmor/zeus.d/*; do
		install -Dm644 -t "$(DESTDIR)/etc/apparmor.d/zeus.d" "$$i"
	done

	install -Dm644 completions/zeus.bash "$(DESTDIR)/usr/share/bash-completion/completions/zeus"
	install -Dm644 completions/zeus.zsh "$(DESTDIR)/usr/share/zsh/site-functions/_zeus"
	install -Dm644 completions/zeus.fish "$(DESTDIR)/usr/share/fish/vendor_completions.d/zeus.fish"

	mkdir -p "$(DESTDIR)/$(PREFIX)/lib/zeus/runtimes"
	chmod 0755 "$(DESTDIR)/$(PREFIX)/lib/zeus/runtimes"

	install -Dm0755 -t "$(DESTDIR)/$(PREFIX)/share/zeus" target/$(BUILD_TYPE)/builder

	for rtlib in target/$(BUILD_TYPE)/librt_*.so; do
		install -Dm644 -t "$(DESTDIR)/$(PREFIX)/lib/zeus/runtimes" "$$rtlib"
	done

	for rtdata in runtimes/*/data/; do
		install -D -t "$(DESTDIR)/$(PREFIX)/share/zeus" "$$rtdata"/*
	done

.PHONY:
apparmor_test:
	-apparmor_parser -R /etc/apparmor.d/zeus
	-cp -r apparmor/* /etc/apparmor.d/
	-aa-complain /etc/apparmor.d/zeus

.PHONY:
uninstall:
	-apparmor_parser -R /etc/apparmor.d/zeus

	-rm -f "$(DESTDIR)/$(PREFIX)/bin/zeus"
	-rm -rf "$(DESTDIR)/$(PREFIX)/share/zeus"
	-rm -rf "$(DESTDIR)/$(PREFIX)/lib/zeus"

	-rm -f "$(DESTDIR)/etc/apparmor.d/zeus"
	-rm -f "$(DESTDIR)/usr/share/bash-completion/completions/zeus"
	-rm -f "$(DESTDIR)/usr/share/zsh/site-functions/_zeus"
	-rm -f "$(DESTDIR)/usr/share/fish/vendor_completions.d/zeus.fish"

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
