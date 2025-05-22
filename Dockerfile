FROM rust:1.87.0-bookworm AS build-env
LABEL maintainer="yanorei32"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

WORKDIR /usr/src
COPY . /usr/src/nf-energy-monitor/
WORKDIR /usr/src/nf-energy-monitor
RUN cargo build --release && cargo install cargo-license && cargo license \
	--authors \
	--do-not-bundle \
	--avoid-dev-deps \
	--avoid-build-deps \
	--filter-platform "$(rustc -vV | sed -n 's|host: ||p')" \
	> CREDITS

FROM debian:bookworm-slim@sha256:90522eeb7e5923ee2b871c639059537b30521272f10ca86fdbbbb2b75a8c40cd

RUN apt-get update; \
	apt-get install -y --no-install-recommends \
		libssl3 ca-certificates; \
	apt-get clean;

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/nf-energy-monitor/CREDITS \
	/usr/src/nf-energy-monitor/LICENSE \
	/usr/share/licenses/nf-energy-logger/

COPY --chown=root:root --from=build-env \
	/usr/src/nf-energy-monitor/target/release/nf-energy-logger \
	/usr/bin/nf-energy-logger

CMD ["/usr/bin/nf-energy-logger"]
