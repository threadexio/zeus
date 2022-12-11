<img src="img/out/optimized.social-image.svg" class="rounded">

<br>

<div align="center">
  <a href="https://github.com/threadexio/zeus/releases/latest">
   <img src="https://img.shields.io/github/v/release/threadexio/zeus?color=%238003fcff&label=version&style=for-the-badge">
  </a>
  <a href="https://github.com/threadexio/zeus/issues">
   <img src="https://img.shields.io/github/issues/threadexio/zeus?style=for-the-badge">
  </a>
  <a href="https://github.com/threadexio/zeus/actions/workflows/build.yaml">
   <img src="https://img.shields.io/github/workflow/status/threadexio/zeus/Build?style=for-the-badge">
  </a>
  <a href="https://github.com/threadexio/zeus/blob/master/LICENSE">
   <img src="https://img.shields.io/github/license/threadexio/zeus?color=%230176ccff&style=for-the-badge">
  </a>

  <img src="https://img.shields.io/static/v1?label=built with&message=rust&color=tan&style=for-the-badge"/>
</div>

<br>
<br>

<h1>
 <img src="img/out/optimized.logo.svg" height=25> What is zeus?
</h1>

**zeus** is a simple AUR helper which utilizes containers allowing developers and users alike to benefit from it's reproducible, clean and flexible builds.

# Contents

* [1. Why zeus?](#why-zeus)
  * [1.1 Everything builds in a container](#everything-builds-in-a-container)
  * [1.2 You can ensure reliable builds](#you-can-ensure-reliable-builds)
* [2. Installation](#installation)
  * [2.1 Install on Archlinux](#install-on-archlinux)
    * [2.1.1 From the AUR](#from-the-aur)
    * [2.1.2 With pacman](#with-pacman)
  * [2.2 Install on other distros](#install-on-other-distros)
* [3. Getting started](#getting-started)
  * [3.1 Concepts](#concepts)
  * [3.2 Usage](#usage)
    * [3.2.1 Building the builder](#building-the-builder)
    * [3.2.2 Sync](#sync)
    * [3.2.3 Remove](#remove)
    * [3.2.4 Query](#query)
* [4. License](#license)

# Why zeus?

* [x] Do you hate having messy packages leaving stray files?
* [x] Do you maintain packages and want to ensure they will always build on a default system?
* [x] Do you want an over-engineered AUR helper?

If you answered yes to any of the previous questions, then **zeus** might be of some use to you.

How can **zeus** help you?

## Everything builds in a container

This means cleaning up your build environment is as easy as removing that container.

## You can ensure reliable builds

The container is just the base image. If your package doesn't build there, then it needs some tweaking.

# Installation

After you install **zeus**, head down to the [usage section](#usage) and read carefully.

## Install on Archlinux

### From the AUR

If you already have another AUR helper it is highly recommended you choose this method over the other. The `zeus-bin` packages installs binaries built in GitHub Actions, you can view the build [here](https://github.com/threadexio/zeus/actions/workflows/build.yaml). The **zeus** package gets the latest release and builds it locally on your machine.

<a href="https://aur.archlinux.org/packages/zeus" target="_blank">
 <img src="https://img.shields.io/aur/version/zeus?label=zeus&logo=archlinux&style=for-the-badge" alt="zeus">
</a>
<a href="https://aur.archlinux.org/packages/zeus-bin" target="_blank">
 <img src="https://img.shields.io/aur/version/zeus-bin?label=zeus-bin&logo=archlinux&style=for-the-badge" alt="zeus-bin">
</a>

### With pacman

Grab the latest package from the [releases page](https://github.com/threadexio/zeus/releases/latest) and install it with `pacman -U`.

> NOTE: If you choose this method then you will have to come back and redo this process for every new version.

## Install on other distros

**zeus** is build the mindset of using as few dependencies as possible, it only has 4 _real_ dependencies:

* [ ] A working computer
* [ ] A POSIX compatible filesystem (ext4, btrfs, xfs, ...)
* [ ] `systemd-sysusers`
* [ ] A supported container solution

The tarball used for the arch package can be found in the [releases page](https://github.com/threadexio/zeus/releases/latest).

# Getting started

## Concepts

### Data directory

The data directory (`/usr/share/zeus`) holds anything **zeus** or its runtimes might need, for example, the docker runtime keeps there the Dockerfile that specifies how to builder container should be built. Inside that directory is also a program called `builder`, you should not run this program directly, instead it is there for runtimes to include inside the containers they build. This program is what actually builds everything inside the container.

### Build directory

The build directory is a directory with special permissions owned by the `zeus` user and it holds all of the synced packages. The default location for the build directory is `/var/cache/aur`. The build directory contains all information **zeus** needs. If you ever need to reset **zeus** you can do so by removing that directory and reinstalling the package, this will automatically setup the correct permissions, UIDs and GIDs.

### Config file

The configuration file, located at `/etc/zeus/zeus.toml`, contains the default options zeus will use. Most of the options in that file can be configured from the command line but for ease of use they are also loaded from this file. The file uses the `toml` configuration format, all configurable options can be found inside the file with comments describing what they do.

An options that is defined in the configuration file will always be overridden if it is also set in the command line. For example, if in the configuration file the options `zeus.color` is set to `never` and you run zeus something like this:

```bash
zeus --color always ...
```

Then `always` will override `never` that was set in the config file.

### Runtimes

Runtimes are how **zeus** can utilize different frameworks to build packages. A runtime a bit like a device driver, only it doesn't interface with hardware. Runtimes provide a simple interface for **zeus** that can:

* create a new image
* create a new container
* start/stop a container

And more. These fundamental operations are then combined to create the complex actions that are required.

Runtimes are packaged in with **zeus** and can be found in `/usr/lib/zeus/runtimes` as `.so` files, or shared objects. These files are loaded inside **zeus** and contain the code that provides the above interface. Runtimes follow the naming scheme: `librt_<framework name>.so`

One example runtime is the docker runtime that simply forwards operations to the docker program (`/usr/bin/docker`), just like the docker commands you run yourself, only a bit more complicated.

## Usage

Before you do anything, you must:

* make sure to have installed the framework you are going to use, for example docker. **zeus** does _not_ install it for you.
* have built the builder with: `zeus -B`

If you have done the above, you are good to go.

<div align="center">
  <img src="img/usage/help.png" width=90%>
</div>

### Building the builder

<div align="center">
  <img src="img/usage/build-help.png" width=90%>
</div>

> NOTE: You should rebuild the builder every time you update your main system in order to avoid dependency conflicts between host-builder.

### Sync

<div align="center">
  <img src="img/usage/sync-help.png" width=90%>
</div>

### Remove

<div align="center">
  <img src="img/usage/remove-help.png" width=90%>
</div>

### Query

<div align="center">
  <img src="img/usage/query-help.png" width=90%>
</div>

# License

All source code for **zeus** is licensed under the [GNU General Public License Version 3](./LICENSE).

All art, logos, images found in this repository are licensed under the [Creative Commons Attribution-NonCommercial 4.0 International Public License](img/LICENSE).

Copies of these licenses can be found inside the respective directories.
