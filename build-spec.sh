#!/usr/bin/env bash
# Generate the plain and raw chain specifications for the Xcavate solochain testnet.
#
# Usage:
#   chmod +x build-spec.sh
#   ./build-spec.sh
#
# The raw chainspec (chainspec/xcavate-testnet.raw.json) is what you upload to
# OnFinality / point nodes at with `--chain`.

set -e

BIN=./target/release/xcavate-node
CHAIN=xcavate-testnet
OUT=chainspec

mkdir -p "$OUT"

# Build the node if it is not already built.
if [ ! -x "$BIN" ]; then
	echo "🚀 Building the Xcavate solochain node..."
	cargo build --release -p xcavate-node
fi

# Generate the human-readable (plain) chain specification.
echo "📝 Generating plain chainspec..."
"$BIN" build-spec --chain "$CHAIN" --disable-default-bootnode > "$OUT/xcavate-testnet.plain.json"

# Generate the raw chain specification (the one nodes actually consume).
echo "🧱 Generating raw chainspec..."
"$BIN" build-spec --chain "$OUT/xcavate-testnet.plain.json" --disable-default-bootnode --raw \
	> "$OUT/xcavate-testnet.raw.json"

echo "✅ Done:"
echo "   $OUT/xcavate-testnet.plain.json"
echo "   $OUT/xcavate-testnet.raw.json"
