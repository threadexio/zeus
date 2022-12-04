FROM archlinux/archlinux:base-devel

RUN pacman --noconfirm --needed -Syu git make rustup

RUN useradd -m builder
RUN echo "builder ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/builder
USER builder

WORKDIR /repo

CMD ["make", "pkg"]
