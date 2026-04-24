# Using github actions for the first time

Im not even using then, cargo-dist just generated all of it

Anyways here is what I understood about it:

first install `dist` (and optionally `cargo release`)

```sh
cargo install cargo-dist
```

```sh
cargo install cargo-release
```

Make sure you have a github repo

```sh
git remote add origin https://github.com/Barni228/github-actions.git
```

And make sure the repo is in `Cargo.toml`:

```toml
[package]
name = "github-actions"
version = "0.1.0"
description = "Using cargo-dist and cargo-release with github actions" # need this for cargo-release
license = "MIT" # need this for cargo-release
edition = "2024"
repository = "https://github.com/Barni228/github-actions" # need this for dist
```

Then in your rust project, run

```
dist init
```

For which platforms to build for, just leave it as default (default has these selected)

```
Apple Silicon macOS (aarch64-apple-darwin)  # Apple mac
ARM64 Linux (aarch64-unknown-linux-gnu)     # Weird linux devices (can be skipped)
Intel macOS (x86_64-apple-darwin)           # Old Apple mac
x64 Linux (x86_64-unknown-linux-gnu)        # Normal linux devices
x64 Windows (x86_64-pc-windows-msvc)        # Windows
```

When it asks `what installers do you want to build?`, if you want other people to install your tool then choose shell and powershell

```
shell
powershell
```

If it asks if it should generate github actions (or `CI`), choose `yes`

If something goes wrong, fix it and run `dist init` again (`dist init` is made to be re-run a bunch of times, every time you want to change something)

Then, to release ur thing to GitHub, first tell `cargo-release` to not publish to crates.io

```sh
echo publish = false > release.toml
```

Then run `cargo-release`, by default it will do a dry-run (just tell you what it will do)

```sh
cargo release
```

You can also increase your version number (new git tag, and will update `Cargo.toml` `version`) by using one of these

```sh
cargo release patch  # Increment by 0.0.1, a bug fix
cargo release minor  # Increment by 0.1.0, a new feature
cargo release major  # Increment by 1.0.0, a breaking change
```

When you are ready, just add `--execute` flag to `cargo-release`
It will tell it to actually release ur thing to github

```sh
cargo release --execute
```

Then in GitHub, go to your repo, Click ` Tag` (or `Tags`)
In there, you will find version numbers, click the most recent one
And Done!, It will show you a `curl` and `powershell` command to install the cli app
As well as showing a table with every binary and which os it is for

Best way to download the app is with the `curl` command, which works on linux mac and windows git-bash

To uninstall the app, run

```sh
which app_name
```

And then delete that path:

```sh
rm path/to/app
```

Or a fancy shortcut:

```sh
which app_name   # make sure that this printed a good path
rm $(!!)         # remove the path that previous command printed
```

Or if you like to delete stuff without any idea of what ur deleting:

```sh
rm $(which app_name)
```
