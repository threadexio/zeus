MAKEFLAGS += --no-builtin-rules --no-builtin-variables --no-print-directory --no-keep-going

undefine CARGO_TARGET_DIR

V ?= 0
ifeq ($(V),1)
	Q :=
else
	Q := @
endif

export V Q

repo        := $(patsubst %/,%,$(dir $(realpath $(lastword $(MAKEFILE_LIST)))))
DESTDIR     ?= /
O           ?= $(repo)/target
package-dir := $(repo)/pkg

export repo O DESTDIR

PROFILE      ?= dev
profile-dir  := $(repo)/profiles
profile-path := $(profile-dir)/$(PROFILE).env
profiles     := $(sort $(basename $(notdir $(wildcard $(profile-dir)/*.env))))

ifeq ($(filter $(PROFILE),$(profiles)),)
$(error Invalid profile '$(PROFILE)', available profiles: $(profiles))
endif

export PROFILE profile-path

CARGO            ?= cargo
CARGO_ARGS       := --target-dir $(O)
CARGO_BUILD_ARGS := --profile $(PROFILE)

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
	$(Q)turboinstall -p $(profile-path) -- $(DESTDIR) $(repo)/overlay
	$(Q)find runtimes/ \
		-maxdepth 2 -type d -name overlay \
		-exec turboinstall -p $(profile-path) -- $(DESTDIR) {} \;

%pkg: FORCE
	$(Q)$(MAKE) -C $(package-dir) $@

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
	@echo ''
	@echo 'Packaging targets:'
	@$(MAKE) -C $(package-dir) help
	@echo ''
	@echo 'Environment:'
	@echo '  V=0|1         - Be verbose, set to show all commands'
	@echo '  CARGO         - Path to the cargo executable (default: $(CARGO))'
	@echo '  CARGO_JOBS    - Number of build jobs to create (default: $(CARGO_JOBS))'
	@echo '  PROFILE       - Build PROFILE (default: $(PROFILE))'
	@echo '                  [possible values: $(profiles)]'

.PHONY: $(PHONY)
