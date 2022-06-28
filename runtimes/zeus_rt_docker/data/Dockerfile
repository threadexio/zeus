FROM archlinux/archlinux:base-devel

RUN pacman -Syyu \
	--noconfirm \
	--needed \
	gcc make cmake git

RUN useradd -r -m builder
RUN echo "builder ALL=(ALL:ALL) NOPASSWD: /usr/bin/pacman" > /etc/sudoers.d/builder

VOLUME [ "/build" ]

COPY --chown=root:root builder /usr/local/bin

RUN chmod +x /usr/local/bin/*

USER builder

WORKDIR /build
ENTRYPOINT ["/usr/local/bin/builder"]
