MAKEFLAGS += --no-builtin-rules --warn-undefined-variables --no-keep-going --no-print-directory

CARGO ?= cargo
CARGO_ARGS ?=

override CARGO_JOBS := -j$(shell expr $(shell nproc) + 2)

PROFILE ?= dev
export PROFILE

DESTDIR ?=
export DESTDIR

CARGO_ARGS += --profile $(PROFILE)

###
### Recipes
###

all: build

check:
	$(CARGO) clippy

build:
	$(CARGO) build $(CARGO_JOBS) $(CARGO_ARGS) --all-features --workspace

clean:
	$(CARGO) clean

test: build
	$(CARGO) test $(CARGO_JOBS) $(CARGO_ARGS) --all-features --workspace

completions:
	./build/zeus completions -s bash > overlay/usr/share/bash-completion/completions/zeus
	./build/zeus completions -s fish > overlay/usr/share/fish/vendor_completions.d/zeus.fish
	./build/zeus completions -s zsh  > overlay/usr/share/zsh/site-functions/_zeus

install: build
	./scripts/install.sh

tar: build
	fakeroot ./scripts/tar.sh

# Build is not PHONY because we can use the symlink
# (/build) from build.rs as a build condition.
.PHONY: all check clean test completions install tar

###
### Flows
###

ci-flow: clean build test tar

.PHONY: ci-flow
