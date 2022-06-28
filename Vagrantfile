# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "archlinux/archlinux"
  config.vm.box_check_update = true

  config.vm.synced_folder ".", "/zeus",
  sshfs_opts_append: "-o ro -o idmap=user",
  type: "sshfs"

  config.vm.provision "shell", inline: <<-SHELL
     pacman --needed --noconfirm -Syu base-devel docker
     gpasswd -a vagrant docker
     sudo systemctl enable docker.service
  SHELL
end
