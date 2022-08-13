[repo]: https://github.com/threadexio/zeus

[latest-release]: https://github.com/threadexio/zeus/releases/latest
[release-badge]: https://img.shields.io/github/v/release/threadexio/zeus?style=for-the-badge&display_name=release

[issues]: https://github.com/threadexio/zeus/issues
[issues-badge]: https://img.shields.io/github/issues-raw/threadexio/zeus?style=for-the-badge

[build]: https://github.com/threadexio/zeus/actions/workflows/build.yaml
[build-badge]: https://img.shields.io/github/workflow/status/threadexio/zeus/Build?style=for-the-badge

[license]: https://github.com/threadexio/zeus/blob/master/LICENSE
[license-badge]: https://img.shields.io/github/license/threadexio/zeus?style=for-the-badge

[pkg-aur]: https://aur.archlinux.org/packages/zeus
[pkg-aur-badge]: https://img.shields.io/aur/version/zeus?style=for-the-badge&label=AUR
[pkg-bin-aur]: https://aur.archlinux.org/packages/zeus-bin
[pkg-bin-aur-badge]: https://img.shields.io/aur/version/zeus-bin?style=for-the-badge&label=AUR

[help]: https://github.com/threadexio/zeus/pulls
[help-badge]: https://img.shields.io/badge/HELP-WANTED-green?style=for-the-badge&logo=github

[wiki]: https://github.com/threadexio/zeus/wiki
[usage]: https://github.com/threadexio/zeus/wiki/Usage
[faq]: https://github.com/threadexio/zeus/wiki/FAQ
[releases]: https://github.com/threadexio/zeus/releases
[ci]: https://github.com/threadexio/zeus/actions
[installing]: #installing
[building]: #building

<!---->

<div align="center">

<img src="assets/logo.optimized.svg" width="25%"/>

<h1>
	<b>zeus</b>
</h1>


**<kbd>[Wiki][wiki]</kbd>** &nbsp; &nbsp;
**<kbd>[Install][installing]</kbd>** &nbsp; &nbsp;
**<kbd>[Usage][usage]</kbd>** &nbsp; &nbsp;
**<kbd>[FAQ][faq]</kbd>** &nbsp; &nbsp;
**<kbd>[Releases][releases]</kbd>** &nbsp; &nbsp;
**<kbd>[Issues][issues]</kbd>** &nbsp; &nbsp;
**<kbd>[CI][ci]</kbd>** &nbsp; &nbsp;

---

[![release-badge]][releases]
[![issues-badge]][issues]
[![build-badge]][build]
[![license-badge]][license]
[![help-badge]][help]

</div>

---

<br>

**Zeus**. A simple AUR helper which utilizes containers allowing developers and users alike to benefit from it's reproducible, clean and flexible builds. To get started with `zeus` follow the [install instructions][installing] or [build it yourself][building]. Be sure to check out the [wiki][wiki] for anything else.


<br>

## Installing

Currently there are 2 packages in the AUR.

-   `zeus` - Which builds from the [latest release][latest-release]
-   `zeus-bin` - Which unpacks prebuilt binaries from the [latest release][latest-release].

|  Package   |               Version               |
| :--------: | :---------------------------------: |
|   `zeus`   |     [![pkg-aur-badge]][pkg-aur]     |
| `zeus-bin` | [![pkg-bin-aur-badge]][pkg-bin-aur] |

> **NOTE:** The binaries for `zeus-bin` are built in [Github Actions][build]

After installing one of the 2 packages, there is one final step towards getting up and running.

Building the actual builder container.

```shell
$ zeus -B
```

> If your user does _**not**_ have access to the docker socket, you will have to run the previous command as root and subsequently every time you want to use the program.

<br>

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

Testing local changes can be done in 2 ways.

<br>

### Not installing locally

This method involves no extra steps.

Running the built binary is as simple as:

```shell
$ ./target/$BUILD_TYPE/zeus
```

<br>

### Installing locally

Installing locally for easier testing is possible with the `install` target.

```shell
# make install
```

> `DESTDIR` and `PREFIX` can be used to alter the installation.

After all this you should be able to just run `zeus` directly in the terminal.

```shell
$ zeus
```
