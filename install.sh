#!/usr/bin/env sh

set -eu

if [ -n "${GITHUB_ACTIONS-}" ]; then
  set -x
fi

# Check pipefail support in a subshell, ignore if unsupported
# shellcheck disable=SC3040
(set -o pipefail 2> /dev/null) && set -o pipefail

help() {
  cat <<'EOF'
Install a binary release of a shai hosted on GitHub

USAGE:
    install.sh [options]

FLAGS:
    -h, --help      Display this message
    -f, --force     Force overwriting an existing binary

OPTIONS:
    --tag TAG       Tag (version) of the crate to install, defaults to latest release
    --to LOCATION   Where to install the binary [default: ~/bin]
    --target TARGET Target triple, see list below:

TARGETS:
    aarch64-apple-darwin        ARM64 macOS
    aarch64-pc-windows-msvc     ARM64 Windows
    aarch64-unknown-linux-musl  ARM64 Linux
    x86_64-apple-darwin        x86_64 macOS
    x86_64-pc-windows-msvc     x86_64 Windows
    x86_64-unknown-linux-musl  x86_64 Linux
EOF
}

crate=shai
url=https://github.com/y1j2x34/shai
releases=$url/releases

say() {
  echo "install: $*" >&2
}

err() {
  if [ -n "${td-}" ]; then
    rm -rf "$td"
  fi

  say "error: $*"
  exit 1
}

need() {
  if ! command -v "$1" > /dev/null 2>&1; then
    err "need $1 (command not found)"
  fi
}

download() {
  url="$1"
  output="$2"

  if command -v curl > /dev/null; then
    curl --proto =https --tlsv1.2 -sSfL "$url" "-o$output"
  else
    wget --https-only --secure-protocol=TLSv1_2 --quiet "$url" "-O$output"
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
    --target)
      target=$2
      shift
      ;;
    --to)
      dest=$2
      shift
      ;;
    *)
      say "error: unrecognized argument '$1'. Usage:"
      help
      exit 1
      ;;
  esac
  shift
done

command -v curl > /dev/null 2>&1 ||
  command -v wget > /dev/null 2>&1 ||
  err "need wget or curl (command not found)"

need mkdir
need mktemp

if [ -z "${tag-}" ]; then
  need grep
  need cut
fi

if [ -z "${target-}" ]; then
  need cut
fi

if [ -z "${dest-}" ]; then
  dest="$HOME/bin"
fi

if [ -z "${tag-}" ]; then
  tag=$(
    download https://api.github.com/repos/y1j2x34/shai/releases/latest - |
    grep tag_name |
    cut -d'"' -f4
  )
fi

if [ -z "${target-}" ]; then
  kernel=$(uname -s)
  arch=$(uname -m)

  case "$kernel" in
    Darwin)
      case "$arch" in
        arm64) target="aarch64-apple-darwin";;
        x86_64) target="x86_64-apple-darwin";;
      esac
      ;;
    Linux)
      case "$arch" in
        aarch64) target="aarch64-unknown-linux-musl";;
        x86_64) target="x86_64-unknown-linux-musl";;
      esac
      ;;
    MINGW* | MSYS* | Windows_NT)
      case "$arch" in
        aarch64) target="aarch64-pc-windows-msvc";;
        x86_64) target="x86_64-pc-windows-msvc";;
      esac
      ;;
    *)
      err "unsupported operating system: $kernel"
      ;;
  esac

  if [ -z "${target-}" ]; then
    err "unsupported architecture: $arch"
  fi
fi

case $target in
  *-pc-windows-msvc) extension=zip; need unzip;;
  *) extension=tar.gz; need tar;;
esac

archive="$releases/download/$tag/$crate-$tag-$target.$extension"

say "Repository:  $url"
say "Crate:       $crate"
say "Tag:         $tag"
say "Target:      $target"
say "Destination: $dest"
say "Archive:     $archive"

td=$(mktemp -d || mktemp -d -t tmp)

if [ "$extension" = "zip" ]; then
  download "$archive" "$td/shai.zip"
  unzip -d "$td" "$td/shai.zip"
else
  download "$archive" - | tar -C "$td" -xz
fi

if [ -e "$dest/shai" ] && [ "$force" = false ]; then
  err "\`$dest/shai\` already exists"
else
  mkdir -p "$dest"
  cp "$td/shai" "$dest/shai"
  chmod 755 "$dest/shai"
fi

rm -rf "$td"
