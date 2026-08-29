#!/usr/bin/env bash
set -euo pipefail

readonly REPOSITORY="CassianoThurow/dustoff"
readonly BINARY_NAME="dustoff"
readonly INSTALL_DIR="${DUSTOFF_INSTALL_DIR:-${HOME}/.local/bin}"

fail() {
  printf 'dustoff installer: %s\n' "$1" >&2
  exit 1
}

command -v curl >/dev/null 2>&1 || fail "curl is required"
command -v sha256sum >/dev/null 2>&1 || fail "sha256sum is required"
command -v tar >/dev/null 2>&1 || fail "tar is required"

case "$(uname -s)" in
  Linux) ;;
  *) fail "only Linux is currently supported" ;;
esac

case "$(uname -m)" in
  x86_64 | amd64) target="x86_64-unknown-linux-gnu" ;;
  aarch64 | arm64) target="aarch64-unknown-linux-gnu" ;;
  *) fail "unsupported architecture: $(uname -m)" ;;
esac

version="${DUSTOFF_VERSION:-latest}"
archive="${BINARY_NAME}-${target}.tar.gz"

if [[ "$version" == "latest" ]]; then
  base_url="https://github.com/${REPOSITORY}/releases/latest/download"
else
  [[ "$version" == v* ]] || version="v${version}"
  base_url="https://github.com/${REPOSITORY}/releases/download/${version}"
fi

temporary_dir="$(mktemp -d)"
trap 'rm -rf -- "$temporary_dir"' EXIT

printf 'Downloading Dustoff for %s...\n' "$target"
curl --fail --location --silent --show-error \
  "${base_url}/${archive}" \
  --output "${temporary_dir}/${archive}"
curl --fail --location --silent --show-error \
  "${base_url}/${archive}.sha256" \
  --output "${temporary_dir}/${archive}.sha256"

(
  cd "$temporary_dir"
  sha256sum --check "${archive}.sha256"
  tar --extract --gzip --file "$archive"
)

[[ -f "${temporary_dir}/${BINARY_NAME}" ]] || fail "release archive does not contain ${BINARY_NAME}"
mkdir -p "$INSTALL_DIR"
install -m 0755 "${temporary_dir}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"

printf '\nDustoff was installed at %s/%s\n' "$INSTALL_DIR" "$BINARY_NAME"

case ":${PATH}:" in
  *":${INSTALL_DIR}:"*) printf 'Run it with: dustoff\n' ;;
  *)
    printf '%s is not currently in PATH. Add this line to your shell configuration:\n\n' "$INSTALL_DIR"
    printf '  export PATH="%s:$PATH"\n\n' "$INSTALL_DIR"
    printf 'Then restart your terminal and run: dustoff\n'
    ;;
esac
