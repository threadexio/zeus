FROM archlinux

RUN pacman -Syyu \
	--noconfirm \
	--needed \
	sudo base base-devel gcc make cmake git

RUN useradd -m builder
RUN echo "builder ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/builder

VOLUME [ "/build" ]

COPY ./builder.sh /usr/local/bin/builder.sh

USER builder
ENTRYPOINT /usr/local/bin/builder.sh