FROM rust:1.90.0-bookworm AS build-env
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

FROM debian:bookworm-slim@sha256:7e490910eea2861b9664577a96b54ce68ea3e02ce7f51d89cb0103a6f9c386e0

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
