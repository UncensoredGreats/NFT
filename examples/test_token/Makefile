all: build

build:
	rustup target add wasm32-unknown-unknown
	cargo install candid-extractor
	cargo build --release --target wasm32-unknown-unknown --package test_token
	candid-extractor target/wasm32-unknown-unknown/release/test_token.wasm > test_token.did

run: build
	dfx start --clean --background
	dfx deploy --argument 'record {symbol="TT"; name="Testing Token";}' icrc7

clean:
	dfx stop
	rm -rf target
	rm Cargo.lock
	rm src/icrc7/icrc7.did
