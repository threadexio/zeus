# How to become a contributor

## Respect others

Be respectful towards others, whether they are contributors or just users having an issue. You should always be friendly towards everyone and be open-minded. This of course does not mean you can't disagree with the other person, but doing so in a non-constructive way is not allowed. Their proposal might or might not be a good choice, whatever the case you should always explain why you think it is or isn't the best choice.

## Developing

### Code style

All code submitted must be formatted with `rustfmt`. Any code not formatted this way will not be merged!

### Dependencies

#### Required

- `make`
- `cargo`
- `rust` stable (nightly rust will not work)
- Any POSIX sh compatible shell (ex `bash`, `sh`, etc)

#### Optional

- `vagrant`: Highly recommended to test your changes without affecting your actual system
- `inkscape` and `gimp`: If you would like to edit the artwork

### Build system

Build the code:

```bash
# Debug build
make
# or
make BUILD_TYPE=debug
# or
make BUILD_SPEC=build.config.debug.mk

# Release build
make BUILD_TYPE=release
# or
make BUILD_SPEC=build.config.release.mk
```

Clean the workspace:

```bash
make clean
```

Update the shell completions:

```bash
make completions
```

Install:

```bash
# Install debug build
make install

# Install release build
make BUILD_TYPE=release install

# Install to a local directory
mkdir test_dir
make DESTDIR=./test_dir install
```

Build tar archive:

```bash
# Debug
make tar

# Release
make BUILD_TYPE=release tar
```

### Repo structure

- `/`
  - `.github`: GitHub stuff
  - `.hooks`: Optional git hooks
  - `.vscode`: VS Code configuration
  - `assets`: Logos and repo artwork
  - `rootdir`: Static files that will be included in any install
  - `scripts`: Scripts used by the build system, don't run manually
  - `src`: All of the source code
  - `build.config.*.mk`: Build configuration files
  - `Makefile`: The build system

### Git hooks

There are hooks inside `/.hooks` to make sure that your code passes the minimum checks (correct formatting and builds successfully).

These are optional, but would save lots of time for everyone. To use them run:

```bash
git config core.hookspath ".hooks"
```

### Vagrant

If you want to use vagrant, you must have these plugins installed:

- `vagrant-sshfs`

To test your changes enter the VM with `vagrant ssh` and run:

```bash
cd /zeus
make install
```

Check that `zeus` is correctly installed with:

```bash
zeus --version
```

## How to commit your code

All commits should be signed and verified with GPG and contain the `Signed-off-by` footer. This is good practice generally and can easily be done by adding `-s` and `-S` to your commit command.

Example:

```bash
git commit -s -S -m 'some change'
```

To avoid having to add `-S` every time run:

```bash
git config commit.gpgsign true
```

So the commit would become:

```bash
git commit -s -m 'some change'
```

If you do not wish to create your own GPG key, unsigned commits are also acceptable.

## How to submit your code

Open a pull request here on GitHub and mark it ready for review when you are done with your changes, another contributor will then review your code and merge it if it works correctly.

But before you go and open the pull request, check that another one wanting to make the changes you want does not already exist. This avoids duplicating work and generally informing other people on what you are currently working on is generally a good idea.
