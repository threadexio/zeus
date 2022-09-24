CARGO ?= cargo

CARGO_ARGS ?=

DESTDIR ?=

BUILD_TYPE ?= debug
BUILD_SPEC := build.config.$(BUILD_TYPE).mk

# Check if the spec exists
ifeq ($(wildcard $(BUILD_SPEC)),)
	$(error "$(BUILD_SPEC)" does not exist)
else
	include $(BUILD_SPEC)
endif

MAKEFLAGS += --warn-undefined-variables --no-keep-going --no-print-directory

REQUIRED_CARGO_ARGS := -j$(shell nproc)
CARGO_ARGS += $(REQUIRED_CARGO_ARGS)

COMPLETIONS_BASH := rootdir/usr/share/bash-completion/completions
COMPLETIONS_FISH := rootdir/usr/share/fish/vendor_completions.d
COMPLETIONS_ZSH  := rootdir/usr/share/zsh/site-functions

all: build

build:
	$(CARGO) build --workspace $(CARGO_ARGS) --

clean:
	$(CARGO) clean --

completions:
	$(CARGO) run --bin=zeus -q $(CARGO_ARGS) -- completions --shell=bash > "$(COMPLETIONS_BASH)/zeus"
	$(CARGO) run --bin=zeus -q $(CARGO_ARGS) -- completions --shell=zsh > "$(COMPLETIONS_ZSH)/_zeus"
	$(CARGO) run --bin=zeus -q $(CARGO_ARGS) -- completions --shell=fish > "$(COMPLETIONS_FISH)/zeus.fish"

install:
	./scripts/install.sh

tar:
	./scripts/tar.sh

assets:
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

assets_clean:
	-rm assets/logo.optimized.svg
	-rm assets/*.png

.PHONY: all build clean completions install assets assets_clean
