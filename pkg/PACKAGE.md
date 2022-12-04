# Packaging zeus

This dockerfile allows packaging `zeus` for `pacman`.

0. Building the image

```bash
docker build --pull -t zeus-pkg -- .
docker run -it -v $PWD/../:/repo --name zeus-pkg -- zeus-pkg
```

1. Create the package

```bash
docker start -ai zeus-pkg
```
