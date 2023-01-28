MAKEFLAGS += --no-builtin-rules --warn-undefined-variables --no-keep-going --no-print-directory

CARGO ?= cargo
CARGO_ARGS ?=

PROFILE ?= dev
export PROFILE

DESTDIR ?= /
export DESTDIR

override CARGO_JOBS := -j$(shell expr $(shell nproc) + 2)
override PROFILES := $(patsubst profiles/%.env,%,$(wildcard profiles/*.env))
override PROFILE_FILE := profiles/$(PROFILE).env

CARGO_ARGS += --profile $(PROFILE)

V ?=
ifeq ($(V),)
	Q := @
else
	Q :=
endif

###
### Recipes
###

all: build

check:
	$(Q)$(CARGO) fmt --check --all
	$(Q)$(CARGO) check --all
	$(Q)$(CARGO) clippy

build: $(PROFILE_FILE)
	$(Q)$(CARGO) build $(CARGO_JOBS) $(CARGO_ARGS) --workspace

test: $(PROFILE_FILE)
	$(Q)$(CARGO) test $(CARGO_JOBS) $(CARGO_ARGS) --workspace

clean:
	$(Q)$(CARGO) clean
	$(Q)rm -f -- ./build zeus.tar.gz pkg/*.pkg.tar.zst

install:
	$(Q)./scripts/install.sh

O ?= zeus.tar.gz
tar:
	$(Q)fakeroot ./scripts/tar.sh $(O)

MAKEPKG_ARGS ?= -fC --noconfirm --needed
pkg:
	$(Q)cd pkg && makepkg $(MAKEPKG_ARGS)

completions: build/zeus
	$(Q)$< completions -s bash > overlay/usr/share/bash-completion/completions/zeus
	$(Q)$< completions -s fish > overlay/usr/share/fish/vendor_completions.d/zeus.fish
	$(Q)$< completions -s zsh  > overlay/usr/share/zsh/site-functions/_zeus

.PHONY: all check build test clean install tar pkg completions

build/zeus:
	$(Q)$(MAKE) build

###
### Misc
###

help:
	@echo 'Variables:'
	@echo '  CARGO                      - Path to the cargo executable (current: $(CARGO))'
	@echo '  CARGO_ARGS                 - Extra arguments for cargo (current: $(CARGO_ARGS))'
	@echo '  PROFILE                    - Build profile (current: $(PROFILE)) [possible values: $(PROFILES)]'
	@echo '  DESTDIR                    - Install destination directory (current: $(DESTDIR))'
	@echo '  V                          - Be verbose, set to show all commands'
	@echo ''
	@echo '  O                          - Specify the output archive for `tar` (current: $(O))'
	@echo '  MAKEPKG_ARGS               - Pass other arguments to makepkg (current: $(MAKEPKG_ARGS))'
	@echo ''
	@echo ''
	@echo 'Build targets:'
	@echo '  build                      - Build all binaries and runtimes'
	@echo '  test                       - Build and run tests'
	@echo ''
	@echo 'Clean targets:'
	@echo '  clean                      - Clean all build artifacts'
	@echo ''
	@echo 'Install targets:'
	@echo '  install                    - Install zeus into DESTDIR'
	@echo '  completions                - Generate shell completions'
	@echo '  tar                        - Create a gzipped tarball from the last build'
	@echo '  pkg                        - Package zeus with makepkg'
