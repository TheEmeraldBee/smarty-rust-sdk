#!/usr/bin/make -f

VERSION_FILE := Cargo.toml
SDK_VERSION_FILE := smarty-rust-sdk/Cargo.toml

clean:
	cargo clean
	git checkout "$(VERSION_FILE)"
	git checkout "$(SDK_VERSION_FILE)"

test:
	cargo test

build:
	cargo build

publish: test
	sed -i -r "s/version=\"0\.0\.0\"/version=\"${VERSION}\"/g" "$(SDK_VERSION_FILE)" \
	  && sed -i -r "s/0\.0\.0/${VERSION}/g" "$(VERSION_FILE)" \
	  && cargo publish --package smarty-rust-proc-macro --allow-dirty \
	  && cargo publish --package smarty-rust-sdk --allow-dirty

clippy:
	cargo clippy

international_autocomplete_api:
	RUST_LOG=trace cargo run --example international_autocomplete_api

international_street_api:
	RUST_LOG=trace cargo run --example international_street_api

logger:
	RUST_LOG=trace cargo run --example logger

us_autocomplete_pro_api:
	RUST_LOG=trace cargo run --example us_autocomplete_pro_api

us_extract_api:
	RUST_LOG=trace cargo run --example us_extract_api

us_reverse_geo_api:
	RUST_LOG=trace cargo run --example us_reverse_geo_api

us_street_api:
	RUST_LOG=trace cargo run --example us_street_api

us_zipcode_api:
	RUST_LOG=trace cargo run --example us_zipcode_api

us_enrichment_api:
	RUST_LOG=trace cargo run --example us_enrichment_api

examples: international_autocomplete_api international_street_api logger us_autocomplete_pro_api us_extract_api us_reverse_geo_api us_street_api us_zipcode_api

.PHONY: clean test dependencies package examples clippy international_autocomplete_api international_street_api logger us_autocomplete_pro_api us_extract_api us_reverse_geo_api us_street_api us_zipcode_api us_enrichment_api
