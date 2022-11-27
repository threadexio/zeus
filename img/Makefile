WIDTH ?= 256
HEIGHT ?= 256

all:
	for s in *.svg; do \
		scour \
		-i "$$s" \
		-o "out/optimized.$$s" \
		--enable-id-stripping \
		--strip-xml-space \
		--no-line-breaks \
		--enable-comment-stripping \
		--shorten-ids \
		--remove-descriptive-elements \
		--create-groups; \
	done

	inkscape \
		social-image.svg \
		-o out/social-image.png \
		--export-type=png \
		--export-overwrite \
		--export-area-page \
		-w 1280 -h 640

clean:
	-rm -f out/*
	-mkdir -p out
