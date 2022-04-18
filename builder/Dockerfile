FROM archlinux/archlinux:base-devel

RUN pacman -Syyu \
	--noconfirm \
	--needed \
	sudo gcc make cmake git

RUN useradd -r -m builder
RUN echo "builder ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/builder

VOLUME [ "/build" ]

COPY --chown=root:root builder /usr/local/bin
COPY --chown=root:root package_builder.sh /usr/local/bin

RUN chmod +x /usr/local/bin/*

WORKDIR /build
ENTRYPOINT ["/usr/local/bin/builder"]