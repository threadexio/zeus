[repo]: https://github.com/threadexio/zeus
[commits]: https://github.com/threadexio/zeus/commits
[releases]: https://github.com/threadexio/zeus/releases
[latest-release]: https://github.com/threadexio/zeus/releases/latest
[issues]: https://github.com/threadexio/zeus/issues
[actions]: https://github.com/threadexio/zeus/actions
[build]: https://github.com/threadexio/zeus/actions/workflows/build.yaml
[license]: https://github.com/threadexio/zeus/blob/master/LICENSE
[pkg-aur]: https://aur.archlinux.org/packages/zeus
[pkg-bin-aur]: https://aur.archlinux.org/packages/zeus-bin
[help]: https://github.com/threadexio/zeus/pulls

<!---->

[pkg-bin-aur]: https://aur.archlinux.org/packages/zeus-bin
[build-badge]: https://img.shields.io/github/workflow/status/threadexio/zeus/Build?style=for-the-badge
[release-badge]: https://img.shields.io/github/v/release/threadexio/zeus?style=for-the-badge&display_name=release
[release-commit-badge]: https://img.shields.io/github/commits-since/threadexio/zeus/latest?style=for-the-badge
[license-badge]: https://img.shields.io/github/license/threadexio/zeus?style=for-the-badge
[issues-badge]: https://img.shields.io/github/issues-raw/threadexio/zeus?style=for-the-badge
[pkg-aur-badge]: https://img.shields.io/aur/version/zeus?style=for-the-badge&label=AUR
[pkg-bin-aur-badge]: https://img.shields.io/aur/version/zeus-bin?style=for-the-badge&label=AUR
[help-badge]: https://img.shields.io/badge/HELP-WANTED-green?style=for-the-badge&logo=github

<div align="center">

<img src="assets/logo.optimized.svg" width=250/>

<h1>
	<b>zeus</b>
</h1>

[Releases][releases] &nbsp; | &nbsp; [CI][actions] &nbsp; | &nbsp; [Issues][issues] &nbsp; | &nbsp; [Installing](#installing) &nbsp; | &nbsp; [Building](#building)

[![release-badge]][releases]
[![issues-badge]][issues]
[![build-badge]][build]
[![license-badge]][license]
[![release-commit-badge]][commits]
[![help-badge]][help]

</div>

---

<br>

**Zeus**. An simple AUR helper which utilizes docker containers allowing developers and users alike to benefit from it's reproducible, clean and flexible builds.

## Table of contents

- [Table of contents](#table-of-contents)
- [Installing](#installing)
- [Building](#building)
	- [Not installing locally](#not-installing-locally)
	- [Installing locally](#installing-locally)

## Installing

Currently there are 2 packages in the AUR.

-   `zeus` - Which builds from the [latest release][latest-release]
-   `zeus-bin` - Which unpacks prebuilt binaries from the [latest release][latest-release].

|  Package   |               Version               |
| :--------: | :---------------------------------: |
|   `zeus`   |     [![pkg-aur-badge]][pkg-aur]     |
| `zeus-bin` | [![pkg-bin-aur-badge]][pkg-bin-aur] |

**NOTE:** The binaries for `zeus-bin` are built in [Github Actions][build]

After installing one of the 2 packages, there is one final step towards getting up and running.

Building the actual builder container

```shell
$ zeus -B --force
```

> If your user does _**not**_ have access to the docker socket, you will have to run the previous command as root and subsequently every time you want to use the program.

## Building

After cloning the repository, use the `build` target in the `Makefile` to build everything.

```shell
$ make build
```

> By default the `build` target builds the debug version, if you wish to build the release version set `BUILD_TYPE=release`.

```shell
$ export BUILD_TYPE=release
$ make build
```

Testing local changes can be done in 2 ways:

---

### Not installing locally

This method involves no extra steps.

Running the built binary is as simple as:

```shell
$ ./target/$BUILD_TYPE/zeus
```

> Remember to specify the builder image archive with `--archive ./builder.tar.gz`

### Installing locally

Installing locally for easier testing is possible with the `install` target.

```shell
# make install
```

> Note the `#`, this means the command must be ran as root to work properly

> `DESTDIR` and `PREFIX` can be used to alter the installation.

After all this you should be able to just run `zeus` directly in the terminal.

```shell
$ zeus
```
