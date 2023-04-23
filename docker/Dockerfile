FROM docker.io/archlinux:base-devel

RUN set -eu; \
	pacman --needed --noconfirm -Syy archlinux-keyring; \
	pacman --needed --noconfirm -Syu sudo rustup git fakeroot;

RUN set -eu; \
	useradd -m builduser; \
	echo 'builduser ALL=(ALL:ALL) NOPASSWD: ALL' >> /etc/sudoers.d/builduser; \
	mkdir -p /app/src; \
	chown -R builduser:builduser /app;

USER builduser
ENV CARGO_TARGET_DIR /app/target
RUN set -eu; \
	git config --global --add safe.directory '/app/src'; \
	rustup default stable;

WORKDIR /app/src
COPY --chown=builduser:builduser \
	./ ./

CMD ["cargo", "xtask", "ci"]
