# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "archlinux/archlinux"
  config.vm.box_check_update = true

  config.vm.network "private_network", ip: "192.168.50.2"

  config.vm.synced_folder ".", "/zeus",
  sshfs_opts_append: "-o ro -o idmap=user",
  type: "sshfs"

  config.vm.provision "shell", path: "scripts/provision_vm.sh"

end
