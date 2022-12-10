MAKEFLAGS += --no-builtin-rules --warn-undefined-variables --no-keep-going --no-print-directory

CARGO ?= cargo
CARGO_ARGS ?=

override CARGO_JOBS := -j$(shell expr $(shell nproc) + 2)

PROFILE ?= dev
export PROFILE

override PROFILES := $(patsubst profiles/%.env,%,$(wildcard profiles/*.env))

DESTDIR ?= /
export DESTDIR

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
	$(Q)$(CARGO) clippy

build: profiles/$(PROFILE).env
	$(Q)$(CARGO) build $(CARGO_JOBS) $(CARGO_ARGS) --all-features --workspace

test: profiles/$(PROFILE).env
	$(Q)$(CARGO) test $(CARGO_JOBS) $(CARGO_ARGS) --all-features --workspace

clean:
	$(Q)$(CARGO) clean
	$(Q)rm -f -- ./build zeus.tar.gz

completions: build/zeus
	$(Q)$< completions -s bash > overlay/usr/share/bash-completion/completions/zeus
	$(Q)$< completions -s fish > overlay/usr/share/fish/vendor_completions.d/zeus.fish
	$(Q)$< completions -s zsh  > overlay/usr/share/zsh/site-functions/_zeus

install: build/zeus
	$(Q)./scripts/install.sh

O ?=
tar:
	$(Q)fakeroot ./scripts/tar.sh $(O)

pkg:
	$(Q)cd pkg && \
		makepkg -sfC --noconfirm

.PHONY: all check build clean test completions install tar pkg

build/zeus:
	$(Q)$(MAKE) build

zeus.tar.gz:
	$(Q)$(MAKE) tar

###
### Flows
###

ci-flow:
	$(Q)make PROFILE=release clean test

.PHONY: ci-flow

###
### Misc
###

help:
	@echo 'Variables:'
	@echo '  CARGO                      - Path to the cargo executable (default: cargo)'
	@echo '  CARGO_ARGS                 - Extra arguments for cargo (default: )'
	@echo '  PROFILE                    - Build profile (default: dev) [possible values: $(PROFILES)]'
	@echo '  DESTDIR                    - Install destination directory (default: /)'
	@echo '  V                          - Be verbose, set to show all commands'
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
