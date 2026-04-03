#!/usr/bin/env bash
# build.sh — Unified multi-target build script for Balatro (Bevy 0.18.1)
#
# Usage:
#   ./build.sh                  # native release (current OS)
#   ./build.sh --native         # native release (explicit)
#   ./build.sh --wasm           # WebAssembly (requires wasm-pack / wasm-bindgen-cli)
#   ./build.sh --all            # native + WASM + Linux cross-compile
#   ./build.sh --dev            # native debug build (fast iteration)
#
# Prerequisites:
#   cargo install wasm-bindgen-cli   # for WASM binding generation
#   rustup target add wasm32-unknown-unknown
#   rustup target add x86_64-unknown-linux-gnu  (for Linux cross on macOS/Windows)

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" \
  || { echo "ERROR: Failed to determine project directory"; exit 1; }
BINARY="balatro"
WASM_OUT="dist/wasm"
NATIVE_OUT="dist/native"

build_native() {
    echo "==> Building native release..."
    cargo build --release
    mkdir -p "$NATIVE_OUT"
    if [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "win32" ]]; then
        local bin="target/release/${BINARY}.exe"
    else
        local bin="target/release/${BINARY}"
    fi
    if [[ ! -f "$bin" ]]; then
        echo "ERROR: Binary not found at $bin after build"; exit 1
    fi
    cp "$bin" "$NATIVE_OUT/"
    # Copy resource assets
    cp -r resources "$NATIVE_OUT/"
    echo "    Native build → $NATIVE_OUT/"
}

build_dev() {
    echo "==> Building native debug (fast)..."
    cargo build
    echo "    Run: cargo run"
}

build_wasm() {
    echo "==> Building WebAssembly..."

    # Ensure wasm target is installed
    rustup target add wasm32-unknown-unknown 2>/dev/null || true

    # Build with the wasm-release profile (size-optimised)
    cargo build --profile wasm-release --target wasm32-unknown-unknown

    mkdir -p "$WASM_OUT"

    # Generate JS bindings with wasm-bindgen
    if command -v wasm-bindgen &>/dev/null; then
        wasm-bindgen \
            "target/wasm32-unknown-unknown/wasm-release/${BINARY}.wasm" \
            --out-dir "$WASM_OUT" \
            --target web \
            --no-typescript
    else
        echo "    wasm-bindgen not found; copying raw .wasm file."
        cp "target/wasm32-unknown-unknown/wasm-release/${BINARY}.wasm" "$WASM_OUT/"
    fi

    # Optional: run wasm-opt for further size reduction
    if command -v wasm-opt &>/dev/null; then
        wasm-opt -Oz \
            "$WASM_OUT/${BINARY}_bg.wasm" \
            -o "$WASM_OUT/${BINARY}_bg.wasm" 2>/dev/null || true
    fi

    # Copy assets and generate a minimal index.html loader
    cp -r resources "$WASM_OUT/"
    generate_wasm_html "$WASM_OUT"

    echo "    WASM build → $WASM_OUT/"
    echo "    Serve with: npx serve $WASM_OUT  OR  python3 -m http.server -d $WASM_OUT"
}

generate_wasm_html() {
    local dir="$1"
    cat > "$dir/index.html" << 'HTML'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Balatro</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { background: #0d0e17; display: flex; justify-content: center; align-items: center; height: 100vh; }
    canvas { max-width: 100%; max-height: 100vh; }
    #loading { color: #e8c840; font-family: monospace; font-size: 1.4em; position: absolute; }
  </style>
</head>
<body>
  <div id="loading">Loading Balatro…</div>
  <script type="module">
    import init from './balatro.js';
    async function run() {
      await init();
      document.getElementById('loading').remove();
    }
    run().catch(e => {
      document.getElementById('loading').textContent = 'Failed to load: ' + e;
      console.error(e);
    });
  </script>
</body>
</html>
HTML
}

build_linux() {
    echo "==> Building Linux x86_64 release..."
    rustup target add x86_64-unknown-linux-gnu 2>/dev/null || true
    cargo build --release --target x86_64-unknown-linux-gnu
    local out="dist/linux"
    mkdir -p "$out"
    cp "target/x86_64-unknown-linux-gnu/release/${BINARY}" "$out/"
    cp -r resources "$out/"
    echo "    Linux build → $out/"
}

# ── Main ─────────────────────────────────────────────────────────────────────

cd "$PROJECT_DIR"

case "${1:-}" in
    --native)   build_native ;;
    --dev)      build_dev ;;
    --wasm)     build_wasm ;;
    --linux)    build_linux ;;
    --all)
        build_native
        build_wasm
        echo "==> All builds complete."
        ;;
    "")
        build_native
        ;;
    *)
        echo "Usage: $0 [--native|--dev|--wasm|--linux|--all]"
        exit 1
        ;;
esac
