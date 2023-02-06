MAKEFLAGS += --no-builtin-rules --no-builtin-variables --warn-undefined-variables --no-keep-going
GNUMAKEFLAGS ?=

V ?= 0
ifeq ($(V),1)
	Q :=
else
	Q := @
endif

O ?= target
DESTDIR ?= /

PROFILE ?= dev
profile-dir := profiles
PROFILE_PATH := $(profile-dir)/$(PROFILE).env
profiles := $(notdir $(wildcard $(profile-dir)/*.env))
profiles := $(sort $(basename $(profiles)))

ifeq ($(filter $(PROFILE),$(profiles)),)
$(error Invalid profile '$(PROFILE)', available profiles: $(profiles))
endif

CARGO ?= cargo
CARGO_JOBS ?= $(shell expr $(shell nproc) + 2)
CARGO_BUILD_ARGS := --profile $(PROFILE) -j$(CARGO_JOBS)
CARGO_ARGS := --target-dir $(O)

MAKEPKG ?= makepkg
MAKEPKG_ARGS ?= -fC --noconfirm --needed
MAKEPKG_WORKDIR ?= $(O)/arch-pkg

TARBALL ?= $(O)/tar-pkg/zeus-bin.tar.gz

export O DESTDIR TARBALL PROFILE PROFILE_PATH

# dont mess up the O variable
undefine CARGO_TARGET_DIR
# stop funny behavior with makepkg
undefine SRCDEST SRCPKGDEST BUILDDIR PKGDEST

PHONY += all
all: build
FORCE: ;

PHONY += check
check:
	$(Q)$(CARGO) fmt --check --all
	$(Q)$(CARGO) check --all
	$(Q)$(CARGO) clippy

PHONY += build
build:
	$(Q)$(CARGO) build $(CARGO_ARGS) $(CARGO_BUILD_ARGS) --workspace

PHONY += test
test:
	$(Q)$(CARGO) test $(CARGO_ARGS) --workspace

PHONY += clean
clean:
	$(Q)rm -f ./build
	$(Q)$(CARGO) clean $(CARGO_ARGS)

PHONY += completions
completions:
	$(Q)$(CARGO) run --bin=zeus -q $(CARGO_ARGS) $(CARGO_BUILD_ARGS) -- --config /dev/null --build-dir . completions -s bash > overlay/usr/share/bash-completion/completions/zeus
	$(Q)$(CARGO) run --bin=zeus -q $(CARGO_ARGS) $(CARGO_BUILD_ARGS) -- --config /dev/null --build-dir . completions -s fish > overlay/usr/share/fish/vendor_completions.d/zeus.fish
	$(Q)$(CARGO) run --bin=zeus -q $(CARGO_ARGS) $(CARGO_BUILD_ARGS) -- --config /dev/null --build-dir . completions -s zsh  > overlay/usr/share/zsh/site-functions/_zeus

PHONY += version
version:
	$(Q)scripts/version.sh

PHONY += buildinfo
buildinfo:
	$(Q)scripts/build_info.sh

PHONY += doc
doc:
	$(Q)$(CARGO) doc --no-deps $(CARGO_ARGS) $(CARGO_BUILD_ARGS)

PHONY += alldoc
alldoc:
	$(Q)$(CARGO) doc $(CARGO_ARGS) $(CARGO_BUILD_ARGS)

PHONY += install
install:
	$(Q)scripts/install.sh

PHONY += tar-pkg
tar-pkg:
	$(Q)fakeroot scripts/tar.sh

PHONY += arch-pkg
.ONESHELL:
arch-pkg:
	$(Q)mkdir -p -- "$(MAKEPKG_WORKDIR)"
	$(Q)ln -rsfT -- . "$(MAKEPKG_WORKDIR)/repo"
	$(Q)cd -- "$(MAKEPKG_WORKDIR)"
	$(Q)ln -sfT -- repo/pkg/PKGBUILD PKGBUILD
	$(Q)$(MAKEPKG) $(MAKEPKG_ARGS)

PHONY += help
help:
	@echo 'Generic targets:'
	@echo '  all           - Build all targets with [*]'
	@echo '  check         - Run linting tests'
	@echo '* build         - Build the source'
	@echo '  test          - Run code tests'
	@echo '  clean         - Clean all build artifacts'
	@echo '  completions   - Update completions in source tree'
	@echo '  version       - Output the current version'
	@echo '  buildinfo     - Output the current build info'
	@echo ''
	@echo 'Documentation targets:'
	@echo '  doc           - Generate documentation only for this crate'
	@echo '  alldoc        - Generate documentation for all crates'
	@echo ''
	@echo 'Install targets:'
	@echo '  install       - Install last build'
	@echo '                    DESTDIR="$(DESTDIR)" install root'
	@echo '  tar-pkg       - Create a tarball from the last build'
	@echo '                    TARBALL="$(TARBALL)" output archive'
	@echo '  arch-pkg      - Use makepkg to create a package from the local build'
	@echo '                    MAKEPKG="$(MAKEPKG)" path to makepkg'
	@echo '                    MAKEPKG_ARGS="$(MAKEPKG_ARGS)" makepkg arguments'
	@echo '                    MAKEPKG_WORKDIR="$(MAKEPKG_WORKDIR)" makepkg temp directory'
	@echo ''
	@echo 'Environment:'
	@echo '  V=0|1         - Be verbose, set to show all commands'
	@echo '  CARGO         - Path to the cargo executable (default: $(CARGO))'
	@echo '  CARGO_JOBS    - Number of build jobs to create (default: $(CARGO_JOBS))'
	@echo '  PROFILE       - Build profile (default: $(PROFILE))'
	@echo '                  [possible values: $(profiles)]'

.PHONY: $(PHONY)
