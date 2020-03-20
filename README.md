# intermodal: a 40' shipping container for the Internet

[![Crate](https://img.shields.io/crates/v/imdl.svg)](https://crates.io/crates/imdl)
[![Build](https://github.com/casey/intermodal/workflows/Build/badge.svg)](https://github.com/casey/intermodal/actions)
[![Chat](https://img.shields.io/discord/679283456261226516.svg?logo=discord)](https://discord.gg/HaaT5Qz)

Intermodal is a user-friendly and featureful command-line BitTorrent metainfo utility.  The binary is called `imdl`. At the moment, creation, viewing, and verification of `.torrent` files is supported.

## Manual

- [General](#general)
  - [Installation](#installation)
    - [Supported Operating Systems](#supported-operating-systems)
    - [Pre-built binaries](#pre-built-binaries)
    - [Cargo](#cargo)
  - [Semantic Versioning](#semantic-versioning)
  - [Unstable Features](#unstable-features)
- [Acknowledgments](#acknowledgments)

## General

### Installation

#### Supported Operating Systems

`imdl` supports both unix and Windows. It is tested on Linux, MacOS, and
Windows, but should work on other unix OSs. If it does not, please open an
issue!

#### Pre-built binaries

Pre-built binaries for Linux, macOS, and Windows can be found on
[the releases page](https://github.com/casey/intermodal/releases).

You can use the following command to download the latest binary for Linux,
MacOS or Windows, just replace `DEST` with the directory where you'd like to
install the `imdl` binary:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://imdl.io/install.sh | bash -s -- --to DEST
```

#### Cargo

`imdl` is written in [Rust](https://www.rust-lang.org/) and can be built from
source and installed with `cargo install imdl`. To get Rust, use the
[rustup installer](https://rustup.rs/).

### Semantic Versioning

Intermodal follows [semantic versioning](https://semver.org/).

In particular:

- v0.0.X: Breaking changes may be introduced at any time.
- v0.X.Y: Breaking changes may only be introduced with a minor version number
  bump.
- vX.Y.Z: Breaking changes may only be introduced with a major version number
  bump

### Unstable Features

To avoid premature stabilization and excessive version churn, unstable features
are unavailable unless the `--unstable` / `-u` flag is passed, for example
`imdl --unstable torrent create .`. Unstable features may be changed or removed
at any time.

## Acknowledgments

The formatting of `imdl torrent show` is entirely copied from
[torf](https://github.com/rndusr/torf-cli), an excellent command-line torrent
creator, editor, and viewer.
