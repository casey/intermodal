#!/usr/bin/env bash

set -eu

help() {
  cat <<'EOF'
Install a binary release of `imdl` hosted on GitHub

Usage:
    install [options]

Options:
    -h, --help      Display this message
    -f, --force     Force overwriting an existing binary
    --tag TAG       Tag (version) to install (default <latest release>)
    --to LOCATION   Where to install the binary (default ~/bin)
EOF
}

err() {
  if [ ! -z ${td-} ]; then
    rm -rf $td
  fi

  say_err "ERROR $1"
  exit 1
}

need() {
  if ! command -v $1 > /dev/null 2>&1; then
    err "need $1 (command not found)"
  fi
}

say() {
  echo "install: $1"
}

say_err() {
  say "$1" >&2
}

need cut
need uname

# bash compiled with MINGW (e.g. git-bash, used in github windows runners),
# unhelpfully includes a version suffix in `uname -s` output, so handle that.
# e.g. MINGW64_NT-10-0.19044
kernel=$(uname -s | cut -d- -f1)
uname_target="$(uname -m)-$kernel"

case $uname_target in
  aarch64-Linux) target=aarch64-unknown-linux-musl;;
  arm64-Darwin) target=aarch64-apple-darwin;;
  armv6l-Linux) target=arm-unknown-linux-musleabihf;;
  armv7l-Linux) target=armv7-unknown-linux-musleabihf;;
  x86_64-Darwin) target=x86_64-apple-darwin;;
  x86_64-Linux) target=x86_64-unknown-linux-musl;;
  x86_64-MINGW64_NT) target=x86_64-pc-windows-msvc;;
  x86_64-Windows_NT) target=x86_64-pc-windows-msvc;;
  *)
    # shellcheck disable=SC2016
    err 'Could not determine target from output of `uname -m`-`uname -s`, please use `--target`:' "$uname_target"
  ;;
esac

force=false
while test $# -gt 0; do
  case $1 in
    --force | -f)
      force=true
      ;;
    --help | -h)
      help
      exit 0
      ;;
    --tag)
      tag=$2
      shift
      ;;
    --to)
      dst=$2
      shift
      ;;
    *)
      ;;
  esac
  shift
done

# Dependencies
need basename
need curl
need install
need mkdir
need mktemp
need tar

# Optional dependencies
if [ -z ${tag-} ]; then
  need rev
fi

if [ -z ${dst-} ]; then
  dst="$HOME/bin"
fi

if [ -z ${tag-} ]; then
  tag=$(curl --proto =https --tlsv1.2 -sSf https://api.github.com/repos/casey/intermodal/releases/latest |
    grep tag_name |
    cut -d'"' -f4
  )
fi

archive="https://github.com/casey/intermodal/releases/download/$tag/imdl-$tag-$target.tar.gz"

say_err "Repository:  https://github.com/casey/intermodal"
say_err "Crate:       imdl"
say_err "Tag:         $tag"
say_err "Target:      $target"
say_err "Destination: $dst"
say_err "Archive:     $archive"

td=$(mktemp -d || mktemp -d -t tmp)
curl --proto =https --tlsv1.2 -sSfL $archive | tar -C $td -xz

if [ -e "$dst/imdl" ] && [ $force = false ]; then
  err "imdl already exists in $dst"
else
  mkdir -p $dst
  install -m 755 $td/imdl $dst
fi

rm -rf $td
