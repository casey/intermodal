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

git=casey/intermodal
crate=imdl
bin=imdl
url=https://github.com/casey/intermodal
releases=$url/releases

case `uname -s` in
  Darwin) target=x86_64-apple-darwin;;
  Linux)  target=x86_64-unknown-linux-musl;;
  *)      target=x86_64-pc-windows-msvc;;
esac

say() {
  echo "install: $1"
}

say_err() {
  say "$1" >&2
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
  need cut
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

archive="$releases/download/$tag/$crate-$tag-$target.tar.gz"

say_err "Repository:  $url"
say_err "Crate:       $crate"
say_err "Tag:         $tag"
say_err "Target:      $target"
say_err "Destination: $dst"
say_err "Archive:     $archive"

td=$(mktemp -d || mktemp -d -t tmp)
curl --proto =https --tlsv1.2 -sSfL $archive | tar -C $td -xz

if [ -e "$dst/$bin" ] && [ $force = false ]; then
  err "$bin already exists in $dst"
else
  mkdir -p $dst
  install -m 755 $td/$bin $dst
fi

rm -rf $td
