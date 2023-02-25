out-package-dir := $(O)/package
export out-package-dir

packages := $(notdir $(patsubst %/,%,$(dir $(wildcard */Makefile))))

help:
	@$(foreach pkg,$(packages), \
	printf "  %-13s - " "$(pkg)pkg"; $(MAKE) -C ./$(pkg) help;)

%pkg:
	$(Q)$(MAKE) -C $(patsubst %pkg,%,$@) package
