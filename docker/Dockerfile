FROM docker.io/archlinux:base-devel

RUN set -eu; \
	pacman --needed --noconfirm -Syy archlinux-keyring; \
	pacman --needed --noconfirm -Syu sudo rustup git fakeroot;

RUN set -eu; \
	useradd -m builduser; \
	echo 'builduser ALL=(ALL:ALL) NOPASSWD: ALL' >> /etc/sudoers.d/builduser; \
	mkdir -p /app/src; \
	git config --system --add safe.directory '/app/src'; \
	chown -R builduser:builduser /app;

USER builduser
RUN set -eu; \
	rustup default stable;

WORKDIR /app/src
ENV CARGO_TARGET_DIR /app/target

CMD ["cargo", "xtask", "ci"]
