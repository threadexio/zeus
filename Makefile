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

COMPLETIONS_BASH := rootdir/usr/share/bash-completion/completions/zeus
COMPLETIONS_ZSH  := rootdir/usr/share/zsh/site-functions/_zeus
COMPLETIONS_FISH := rootdir/usr/share/fish/vendor_completions.d/zeus.fish

all: build

build:
	$(CARGO) build --workspace $(CARGO_ARGS) --

clean:
	$(CARGO) clean --

completions:
	$(CARGO) run --bin=zeus -q $(CARGO_ARGS) -- completions --shell=bash > "$(COMPLETIONS_BASH)"
	$(CARGO) run --bin=zeus -q $(CARGO_ARGS) -- completions --shell=zsh > "$(COMPLETIONS_ZSH)"
	$(CARGO) run --bin=zeus -q $(CARGO_ARGS) -- completions --shell=fish > "$(COMPLETIONS_FISH)"

install:
	./scripts/install.sh

tar:
	./scripts/tar.sh

.PHONY: all build clean completions install
