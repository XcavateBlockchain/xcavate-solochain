FROM docker.io/paritytech/ci-unified:latest as builder

WORKDIR /polkadot
COPY . /polkadot

RUN cargo fetch
RUN cargo build --locked --release

FROM docker.io/parity/base-bin:latest

COPY --from=builder /polkadot/target/release/xcavate-node /usr/local/bin

# Bake the frozen testnet chainspec into the image. Genesis is immutable, so run
# nodes against this file (NOT `--chain xcavate-testnet`, which would regenerate):
#   xcavate-node --chain /chainspec/xcavate-testnet.raw.json --validator ...
# Genesis hash: 0x28db0af57b1b0b91697082df4b46ee3062254becd13325ff922b3dd3f65eeb8a
COPY chainspec/xcavate-testnet.raw.json /chainspec/xcavate-testnet.raw.json

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /data /polkadot/.local/share && \
	chown -R polkadot:polkadot /data && \
	ln -s /data /polkadot/.local/share/polkadot && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/xcavate-node --version

USER polkadot

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/xcavate-node"]
