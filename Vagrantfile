# -*- mode: ruby -*-
# vi: set ft=ruby :

$provision_script = <<-EOS
pacman -Syy archlinux-keyring
pacman --needed --noconfirm -Syu base-devel git gdb bash-completion docker
gpasswd -a vagrant docker

cat << EOF > /etc/systemd/system/gdbserver.service
[Unit]
Description=gdbserver

[Service]
ExecStart=/usr/bin/gdbserver --multi '*:5555'

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable gdbserver.service
systemctl enable docker.service

git config --global --add safe.directory /zeus

echo You should reboot the VM now
EOS

Vagrant.configure("2") do |config|
  config.vm.box = "archlinux/archlinux"
  config.vm.box_check_update = true

  config.vm.synced_folder ".", "/zeus",
  sshfs_opts_append: "-o ro -o idmap=user",
  type: "sshfs"

  config.vm.provision "shell", privileged: true, reboot: true, inline: $provision_script
end
