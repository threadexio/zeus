<div align="center">

  <img src="img/logo.svg" width=300 alt="logo image">

  <h1>
  Ζεύς
  </h1>

  <h3>
  /zefs/
  </h3>

  [![GitHub release (latest by date)](https://img.shields.io/github/v/release/threadexio/zeus?style=for-the-badge&labelColor=%23292929&color=%236400bd&logo=github)](https://github.com/threadexio/zeus/releases/latest)
  [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/threadexio/zeus/ci.yaml?style=for-the-badge&labelColor=%23292929&color=%236400bd&logo=github-actions&logoColor=white)](https://github.com/threadexio/zeus/actions/workflows/ci.yaml)
  [![AUR votes](https://img.shields.io/aur/votes/zeus?style=for-the-badge&labelColor=%23292929&color=%23d6d6d6&logo=archlinux&logoColor=white)](https://aur.archlinux.org/packages/zeus)
  [![GitHub](https://img.shields.io/github/license/threadexio/zeus?style=for-the-badge&color=%23d6d6d6&labelColor=%23292929&logo=unlicense&logoColor=%23d6d6d6)](https://github.com/threadexio/zeus)

</div>

<h1>What is zeus?</h1>

zeus is a simple AUR helper which utilizes containers allowing developers and users alike to benefit from it's reproducible, clean and flexible builds.

* [Features](#features)
* [Security](#security)
* [Installation](#installation)
  * [Archlinux](#archlinux)
    * [pacman](#pacman)
    * [AUR](#aur)
  * [Other distros](#other-distros)
* [Usage](#usage)
  * [Help](#help)
  * [Sync](#sync)
  * [Remove](#remove)
  * [Build](#build)
  * [Query](#query)
  * [Runtime](#runtime)
  * [Completions](#completions)
* [Concepts](#concepts)
  * [Data directory](#data-directory)
  * [Build directory](#build-directory)
  * [Config file](#config-file)
  * [Runtimes](#runtimes)
* [License](#license)

# Features

* [x] 🏗 Consistent builds
* [x] 🧱 Build environment separation
* [x] 🛡️ Security from misbehaving build scripts (see [Security](#security))
* [x] 🌈 Colors
* [x] 🐚 Shell completions

# Security

<div style="border: 1px solid rgb(168, 0, 20); background: rgba(168, 0, 20, 0.25); padding: 10px;">

Zeus can _**NOT**_ protect you from malicious packages. It is only as good as the runtime used. For example, if you use `docker` with zeus then, all of the security risks associated with docker apply. And this is only for the build scripts.

A malicious actor that would want to cause damage would never use the build scripts to do so. They would instead use install scriptlets that are executed by `pacman` as root when the package is installed. This is not something zeus can prevent.

</div>

# Installation

## Archlinux

### pacman

If you don't already have another AUR helper, you can grab the prebuilt Arch package from the [releases page](https://github.com/threadexio/zeus/releases/latest) and install it with pacman.

1. Download the file ending in `.pkg.tar.zst`

2. Install it with pacman

```bash
sudo pacman -U ~/Downloads/zeus-*.pkg.tar.zst
```

When it is finished installing, you should have a look at the [Usage](#usage) section and be sure to then install either on of the AUR packages from bellow using `zeus`, otherwise you will have to redo the above procedure every time you want to upgrade `zeus`.

### AUR

* The `zeus` package gets the latest release and builds it locally on your machine.
* The `zeus-bin` package installs binaries built in GitHub Actions, you can view the build [here](https://github.com/threadexio/zeus/actions/workflows/ci.yaml).

[![zeus](https://img.shields.io/aur/version/zeus?style=for-the-badge&labelColor=%23292929&color=%236400bd&label=zeus&logo=archlinux&logoColor=%23d6d6d6)](https://aur.archlinux.org/packages/zeus)
[![zeus-bin](https://img.shields.io/aur/version/zeus-bin?style=for-the-badge&labelColor=%23292929&color=%236400bd&label=zeus-bin&logo=archlinux&logoColor=%23d6d6d6)](https://aur.archlinux.org/packages/zeus-bin)

## Other distros

zeus is build the mindset of using as few dependencies as possible, it only has 4 _real_ dependencies:

* [x] A working computer
* [x] A POSIX compatible filesystem (ext4, btrfs, xfs, ...)
* [x] `systemd-sysusers`
* [x] A supported container runtime

The tarball used to build the arch package can be found in the [releases page](https://github.com/threadexio/zeus/releases/latest). If that tarball is not make properly to package for that distro, please open an issue and I _might_ add official packages for that it.

# Usage

Before you can actually start building packages, you must first set up zeus.

* Install the container runtime you are going to use and configure zeus to use it.

If for example you are going to use docker, you must follow its own installation instructions and then edit the configuration file for zeus (`/etc/zeus/zeus.toml`) and edit the runtime section to look like this:

```toml
[runtime]
Name = "docker"
```

* Because zeus uses containers to build packages, it also needs a base image. Zeus can create this image if you just invoke it like this:

```bash
zeus -B
```

This will use whatever runtime you configured above to build that package.

## Help

<details>
<summary>Command line options</summary>

```bash
Usage: zeus [OPTIONS] <COMMAND>

Commands:
  sync, -S, --sync      Sync packages
  remove, -R, --remove  Remove packages
  build, -B, --build    Build/Update builder
  query, -Q, --query    Query the AUR
  runtime               Various runtime operations
  completions           Generate shell completions

Options:
      --color <when>   Colorize the output [possible values: never, always, auto]
  -l, --level <level>  Set log level [possible values: fatal, error, warn, info, debug, trace]
      --aur <url>      AUR URL
      --rt <name>      Specify runtime to use
      --name <name>    Builder machine name
      --image <image>  Builder machine image
      --config <path>  Set an alternate configuration file [default: /etc/zeus/zeus.toml]
  -h, --help           Print help
  -V, --version        Print version
```

</details>

* `--color` tells zeus if it should output pretty colors.

* `-l` or `--level` tells zeus how much data to output to the screen. Unless debugging the default value is fine.

* `--aur` tells zeus where the AUR is, if there was another instance of the AUR you could set this to `https://aur.example.com/`.

* `--rt` tells zeus which runtime to use.

* `--name` tells zeus which builder to use.

* `--image` tells zeus which image to use for the builder.

* `--config` tells zeus to load another configuration file.

> NOTE: Any options passed in the command line _will_ override the ones in the configuration file.

## Sync

<details>
<summary>Command line options</summary>

```bash
Sync packages

Usage: zeus {sync|--sync|-S} [OPTIONS] [packages]...

Arguments:
  [packages]...  Packages to sync

Options:
  -u, --upgrade            Upgrade packages
      --install            Install packages after build
      --build-args <args>  Extra arguments for makepkg
  -h, --help               Print help
```

</details>

The sync command allows you to download and build packages from the AUR. The built packages can be found in the build directory (`/var/cache/aur`).

* `--upgrade` tells zeus to download any package not found locally but to also try to update any package that _is_ found locally. By default the latter does not happen.

* `--install` tells zeus to run `pacman -U` to install the packages that were built. Be careful with this flag as installing packages from the AUR is not without risks and you should take great care to audit the packages you build and install. The packages can be found in the build directory (`/var/cache/aur`).

* `--build-args` tells zeus to run `makepkg` inside the builder with these extra arguments (you should not really need to modify this).

## Remove

<details>
<summary>Command line options</summary>

```bash
Remove packages

Usage: zeus {remove|--remove|-R} [OPTIONS] [packages]...

Arguments:
  [packages]...  Packages to remove

Options:
      --uninstall  Uninstall packages after remove
  -h, --help       Print help
```

</details>

The remove command allows you to completely remove packages from the build directory.

* `--uninstall` tells zeus to also uninstall the packages from the host with `pacman -R`.

> NOTE: Zeus does not run pacman automatically.

## Build

<details>
<summary>Command line options</summary>

```bash
Build/Update builder

Usage: zeus {build|--build|-B}

Options:
  -h, --help  Print help
```

</details>

The build command tells zeus to create or update the builder.

> NOTE: You should rebuild the builder every time you update your main system in order to avoid dependency conflicts between host-builder.

## Query

<details>
<summary>Command line options</summary>

```bash
Query the AUR

Usage: zeus {query|--query|-Q} [OPTIONS] [keywords]...

Arguments:
  [keywords]...

Options:
  -i, --info             Display additional information on results
  -b, --by <rule>        Query AUR packages by [possible values: name, namedesc, maintainer, depends, makedepends, optdepends, checkdepends]
      --output <format>  Output format [possible values: pretty, json]
  -h, --help             Print help
```

</details>

The query command provides a method of searching the AUR for you and your scripts.

* `--info` tells zeus to display additional information about the packages. This disables the search feature and instead zeus expects package names, not keywords.

* `--by` tells zeus how to match packages from the AUR.

* `--output` tells zeus to output machine readable JSON for use in scripts.

Running this command with no keywords will instead tell zeus to list all the packages currently found locally.

## Runtime

<details>
<summary>Command line options</summary>

```bash
Various runtime operations

Usage: zeus runtime [OPTIONS]

Options:
  -l, --list  List all available runtimes
  -h, --help  Print help
```

</details>

The runtime command provides an interface for humans to test and debug the runtime.

## Completions

<details>
<summary>Command line options</summary>

```bash
Generate shell completions

Usage: zeus completions [OPTIONS]

Options:
  -s, --shell <shell>  Specify shell to generate completions for [possible values: bash, fish, zsh]
  -h, --help           Print help
```

</details>

The completions command generate shell completions for zeus. One should never really need to use this command as completions should be included in the package you installed zeus with. If not please contact the maintainer of that package.

# Concepts

## Data directory

The data directory (`/usr/share/zeus`) holds anything zeus or its runtimes might need, for example, the docker runtime keeps there the Dockerfile that specifies how to builder container should be built. Inside that directory is also a program called `builder`, you should not run this program directly, instead it is there for runtimes to include inside the containers they build. This program is what actually builds everything inside the container and communicates back to the host. Without it the container is useless.

## Build directory

The build directory is a directory with special permissions owned by the `zeus` user and it holds all of the synced packages. The default location for the build directory is `/var/cache/aur`. The build directory contains all information zeus needs.

Right now there is no way to create a new build directory automatically. But you can create one by hand with:

```bash
set build_dir="<path to build directory>"
mkdir "$build_dir"
chown 23248:23248 "$build_dir"
chmod 6770 "$build_dir"
```

## Config file

The configuration file, located at `/etc/zeus/zeus.toml`, contains the default options zeus will use. Most of the options in that file can be configured from the command line but for ease of use they are also loaded from this file. The file uses the `toml` configuration format. All configurable options can be found inside the file with comments describing what they do.

Any option that is defined in the configuration file will always be overridden if it is also set in the command line. For example, if in the configuration file the option `zeus.Color` is set to `never` and you run zeus like this:

```bash
zeus --color always ...
```

Then `always` will override `never` that was set in the config file.

## Runtimes

Runtimes are how zeus can utilize different frameworks to build packages. A runtime a bit like a device driver, only it doesn't interface with hardware. Runtimes provide a simple interface for zeus that can:

* create a new image
* create a new container
* start/stop a container

And more. These fundamental operations are then combined to create the complex actions that are required.

Runtimes that are packaged in with zeus and can be found in `/usr/lib/zeus/runtimes` as `.so` files, or shared objects. These files are loaded inside zeus and contain the code that provides the above interface. Runtimes follow the naming scheme: `librt_<name>.so`

One example runtime is the docker runtime that simply forwards operations to the docker program (`/usr/bin/docker`), just like the docker commands you run yourself, only a bit more complicated.

For developing runtimes please see `src/lib.rs` and the prepackaged runtimes in `runtimes/`, the `zeus_rt_null` is a good place to start as it does nothing but print messages to the screen, it will help you understand how the interface works.

# License

All source code for zeus is licensed under the [GNU General Public License Version 3](./LICENSE).

All art, logos, images found in this repository are licensed under the [Creative Commons Attribution-NonCommercial 4.0 International Public License](img/LICENSE).

Copies of these licenses can be found inside the respective directories.
