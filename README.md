# Runnables CLI

CLI to run executables in a (usually cargo) workspace.

![screenshot](https://raw.githubusercontent.com/mbecker20/runnables-cli/master/screenshot.png)

## Install
`cargo install runnables-cli`

Note. requires Cargo. Install with [rustup](https://rustup.rs/).

## Usage
It's just `run` in the workspace directory.
```shell
cli to run executables in a workspace

Usage: run [OPTIONS] [PATH]

Arguments:
  [PATH]  [default: .]

Options:
  -s, --search <SEARCH>  
  -c, --color <COLOR>    [default: Blue]
  -i, --ignore <IGNORE>  [possible values: none, run-file, shell, rust-bin, rust-lib, javascript]
  -h, --help             Print help
  -V, --version          Print version
```

## What it does:

- Recursively scans the current and any nested directories.
- Picks up any shell scripts, runfile.toml, and reads Cargo.tomls.
- Presents user with the options.

## Cargo projects

All member crates in a workspace will be picked up. It doesn't *need* to be a workspace either, any folder with multiple nested cargo projects (containing Cargo.toml) will be picked up.

They will be split by binary / library crate.

## Runfile

Define specific commands to run in the `runfile.toml`.

```toml
[just-ls]
description = "logs files / folders in current directory"
cmd = "ls -a"

[ls-in-src]
description = "logs files / folders in src directory"
cmd = "ls -a"
path = "src"

[ls-in-example]
description = "logs files / folders in example directory"
cmd = "ls -a"
path = "runnables-example"
```

## .runignore

Since runnables-cli is scanning your directories recursively, folders which are deeply nested, or containing many files can make the cli take a noticable amount of time to load. 

In this case, we can tell runnables to skip the directory by adding a `.runignore` file.

This file contains directories (relative or absolute) to skip.