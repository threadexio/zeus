TARBALL ?= $(out-package-dir)/zeus-bin-$(shell $(repo)/scripts/version.sh).tar.gz
DESTDIR := $(out-package-dir)/tar

help:
	@echo 'Build tar package'
	@echo '                    TARBALL="$(TARBALL)" output tarball'

.ONESHELL:
package:
	$(Q)fakeroot sh << EOS
	set -e

	set -a
	source $(profile-path)
	set +a

	mkdir -p $(DESTDIR)
	$(MAKE) -C $(repo) DESTDIR=$(DESTDIR) install
	tar -acvpf $(TARBALL) -C $(DESTDIR) \
			--no-acls \
			--no-selinux \
			--no-xattrs \
			-- .
	EOS
