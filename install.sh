#!/bin/sh
set -eu

REPO="${TENKI_REPO:-keyork/tenki}"
BIN_NAME="tenki"
VERSION="${TENKI_VERSION:-}"
INSTALL_DIR="${TENKI_INSTALL_DIR:-}"

usage() {
    cat <<'EOF'
tenki installer

Usage:
  sh install.sh [--version <tag>] [--install-dir <dir>]

Options:
  --version <tag>      Install a specific tag (for example: v0.1.0)
  --install-dir <dir>  Install to a specific directory
  -h, --help           Show this help
EOF
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --version)
            [ "$#" -ge 2 ] || { echo "missing value for --version" >&2; exit 1; }
            VERSION="$2"
            shift 2
            ;;
        --install-dir)
            [ "$#" -ge 2 ] || { echo "missing value for --install-dir" >&2; exit 1; }
            INSTALL_DIR="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "unknown argument: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
done

require_cmd() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "required command not found: $1" >&2
        exit 1
    }
}

require_cmd curl
require_cmd tar
require_cmd uname

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux) PLATFORM="unknown-linux-gnu" ;;
    Darwin) PLATFORM="apple-darwin" ;;
    *)
        echo "unsupported OS: $OS" >&2
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64|amd64) CPU="x86_64" ;;
    arm64|aarch64) CPU="aarch64" ;;
    *)
        echo "unsupported architecture: $ARCH" >&2
        exit 1
        ;;
esac

TARGET="${CPU}-${PLATFORM}"

case "$TARGET" in
    x86_64-unknown-linux-gnu|x86_64-apple-darwin|aarch64-apple-darwin) ;;
    *)
        echo "unsupported target: $TARGET" >&2
        echo "currently supported: x86_64 Linux, x86_64 macOS, aarch64 macOS" >&2
        exit 1
        ;;
esac

if [ -z "$VERSION" ]; then
    VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | sed -n 's/^[[:space:]]*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' | head -n 1)"
fi

if [ -z "$VERSION" ]; then
    echo "failed to resolve latest release tag" >&2
    exit 1
fi

if [ -z "$INSTALL_DIR" ]; then
    if [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
    else
        [ -n "${HOME:-}" ] || { echo "HOME is not set; please pass --install-dir" >&2; exit 1; }
        INSTALL_DIR="${HOME}/.local/bin"
    fi
fi

TMP_DIR="$(mktemp -d 2>/dev/null || mktemp -d -t tenki)"
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM

ASSET="${BIN_NAME}-${VERSION}-${TARGET}.tar.gz"
CHECKSUMS="tenki-${VERSION}-checksums.txt"
BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"
ARCHIVE_PATH="${TMP_DIR}/${ASSET}"

echo "Installing ${BIN_NAME} ${VERSION} (${TARGET})..."
curl -fL "${BASE_URL}/${ASSET}" -o "${ARCHIVE_PATH}"

if curl -fsSL "${BASE_URL}/${CHECKSUMS}" -o "${TMP_DIR}/${CHECKSUMS}"; then
    EXPECTED="$(grep " ${ASSET}\$" "${TMP_DIR}/${CHECKSUMS}" | awk '{print $1}' | head -n 1 || true)"
    if [ -n "$EXPECTED" ]; then
        if command -v sha256sum >/dev/null 2>&1; then
            ACTUAL="$(sha256sum "${ARCHIVE_PATH}" | awk '{print $1}')"
        elif command -v shasum >/dev/null 2>&1; then
            ACTUAL="$(shasum -a 256 "${ARCHIVE_PATH}" | awk '{print $1}')"
        else
            ACTUAL=""
        fi

        if [ -n "${ACTUAL}" ] && [ "$ACTUAL" != "$EXPECTED" ]; then
            echo "checksum verification failed" >&2
            exit 1
        fi
    fi
fi

tar -xzf "${ARCHIVE_PATH}" -C "${TMP_DIR}"
BIN_PATH="$(find "${TMP_DIR}" -type f -name "${BIN_NAME}" | head -n 1 || true)"

if [ -z "$BIN_PATH" ]; then
    echo "failed to locate ${BIN_NAME} in downloaded archive" >&2
    exit 1
fi

if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR" 2>/dev/null || true
fi

if [ -w "$INSTALL_DIR" ]; then
    install -m 755 "$BIN_PATH" "${INSTALL_DIR}/${BIN_NAME}"
else
    require_cmd sudo
    sudo install -m 755 "$BIN_PATH" "${INSTALL_DIR}/${BIN_NAME}"
fi

echo "Installed to ${INSTALL_DIR}/${BIN_NAME}"
case ":${PATH}:" in
    *":${INSTALL_DIR}:"*) ;;
    *)
        echo "warning: ${INSTALL_DIR} is not in your PATH" >&2
        ;;
esac

echo "Run: ${BIN_NAME} --version"
