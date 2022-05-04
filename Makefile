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

	export DEFAULT_ARCHIVE="$(PREFIX)/share/zeus/builder.tar.gz"

	export VERSION="$(VERSION)"
	cargo build $(CARGO_ARGS) --

	tar -acvf builder.tar.gz \
		-C $$PWD/builder/               . \
		-C $$PWD/target/$(BUILD_TYPE)/  builder

.PHONY:
clean: FORCE
	-rm builder.tar.gz zeus-bin.tar.gz
	-cargo clean $(CARGO_ARGS) --

.PHONY:
completions: FORCE
	./target/$(BUILD_TYPE)/zeus misc --shell bash > completions/zeus.bash
	./target/$(BUILD_TYPE)/zeus misc --shell zsh > completions/zeus.zsh
	./target/$(BUILD_TYPE)/zeus misc --shell fish > completions/zeus.fish

.PHONY:
package: build
	tar -acvf zeus-bin.tar.gz \
		-C $$PWD/target/$(BUILD_TYPE)/  zeus \
		-C $$PWD/                       builder.tar.gz \
		-C $$PWD/                       completions/ \
		extra/zeus

.PHONY:
install: completions
	install -Dm0755 -t "$(DESTDIR)/$(PREFIX)/bin" target/$(BUILD_TYPE)/zeus
	install -Dm0644 -t "$(DESTDIR)/$(PREFIX)/share/zeus" builder.tar.gz

	mkdir -p "$(DESTDIR)/var/cache/aur"
	chmod 0777 "$(DESTDIR)/var/cache/aur"

	install -Dm644 -t "$(DESTDIR)/etc/apparmor.d" extra/zeus

	install -Dm644 completions/zeus.bash "$(DESTDIR)/usr/share/bash-completion/completions/zeus"
	install -Dm644 completions/zeus.zsh "$(DESTDIR)/usr/share/zsh/site-functions/_zeus"
	install -Dm644 completions/zeus.fish "$(DESTDIR)/usr/share/fish/vendor_completions.d/zeus.fish"

.PHONY:
apparmor_test:
	-apparmor_parser -R /etc/apparmor.d/zeus
	-cp extra/zeus /etc/apparmor.d/zeus
	-aa-enforce /etc/apparmor.d/zeus

.PHONY:
uninstall:
	-rm $(DESTDIR)/$(PREFIX)/bin/zeus
	-rm -ri $(DESTDIR)/$(PREFIX)/share/zeus

	-rm -i $(DESTDIR)/etc/apparmor.d/zeus
	-rm -i "$(DESTDIR)/usr/share/bash-completion/completions/zeus"
	-rm -i "$(DESTDIR)/usr/share/zsh/site-functions/_zeus"
	-rm -i "$(DESTDIR)/usr/share/fish/vendor_completions.d/zeus.fish"

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
