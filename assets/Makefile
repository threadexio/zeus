WIDTH ?= 256
HEIGHT ?= 256

all:
	scour \
		-i logo.inkscape.svg \
		-o logo.optimized.svg \
		--enable-id-stripping \
		--strip-xml-space \
		--no-line-breaks \
		--enable-comment-stripping \
		--shorten-ids \
		--remove-descriptive-elements \
		--create-groups

	inkscape -C -w $(WIDTH) -h $(HEIGHT) \
		-o logo.$(WIDTH)x$(HEIGHT).png \
		--export-type=png \
		logo.inkscape.svg

clean:
	-rm logo.optimized.svg
	-rm *.png
