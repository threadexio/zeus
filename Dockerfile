FROM archlinux

RUN pacman -Syyu \
	--noconfirm \
	--needed \
	sudo base base-devel gcc make cmake git

RUN useradd -r -m builder
RUN echo "builder ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/builder

VOLUME [ "/build" ]

COPY --chown=root:root builder /usr/local/bin
COPY --chown=root:root package_builder.sh /usr/local/bin

RUN chmod +x /usr/local/bin/*

WORKDIR /build
ENTRYPOINT ["/usr/local/bin/builder"]