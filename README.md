# zeus

docker containers, huh?

they are pretty neat, so i thought why not use them in package building... because what could possibly go wrong...

# dear god, why?

sometimes it's good to keep the system clean from those pesky dev dependencies and have somewhat better security

# it has problems

i know.

feel free to open an issue/pull request

# installing

there are 2 packages already in the AUR for this

-   zeus
-   zeus-bin

then run once to build the builder image

```bash
zeus -B --force
```

also do this if you also want to change/upgrade the builder

the `--force` means do not use the cache

for Highly Advanced Users™, you can use use custom images by creating a new tarball with all the required files and supply the tarball path with `--archive`

note that if your user is not in the `docker` group you will have to use sudo

then you can just build packages with

```bash
zeus -S -p package1,package2 -p package3
```

or upgrade existing ones with

```bash
zeus -Su -p package1,package2 -p package3
```

the built packages can be found in `/var/cache/aur`

# building

the `docker_image` makefile recipe will build all the dependencies and the docker image for the package builder

```bash
sudo make BUILD_TYPE=release build docker_image
```
