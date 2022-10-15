FROM archlinux/archlinux:base-devel

RUN pacman -Syyu \
	--noconfirm \
	--needed \
	gcc make cmake git

RUN useradd -r -m builder
RUN echo "builder ALL=(ALL:ALL) NOPASSWD: /usr/bin/pacman" > /etc/sudoers.d/builder

COPY --chown=root:root builder /builder

RUN chmod +x /builder

USER builder

WORKDIR /build
ENTRYPOINT ["/builder"]
