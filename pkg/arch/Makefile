MAKEPKG      ?= makepkg
MAKEPKG_ARGS ?= -fC

help:
	@echo 'Build arch package'
	@echo '                    MAKEPKG="$(MAKEPKG)" path to makepkg'
	@echo '                    MAKEPKG_ARGS="$(MAKEPKG_ARGS)" arguments for makepkg'

.ONESHELL:
package:
	$(Q)sh << EOS
	set -e

	set -a
	source $(profile-path)
	set +a

	$(Q) \
		PKGDEST=$(out-package-dir) \
		SRCDEST=$(out-package-dir)/arch/src \
		SRCPKGDEST=$(out-package-dir)/arch/src \
		LOGDEST=$(out-package-dir)/arch/log \
		BUILDDIR=$(out-package-dir)/arch/pkg \
		$(MAKEPKG) $(MAKEPKG_ARGS)
	EOS
