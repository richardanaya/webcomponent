start:
	cargo update
	cargo +nightly web build --target-webasm
	mkdir demo/js
	cp target/wasm32-unknown-unknown/release/hello-world.js demo/js/app.js
	cp target/wasm32-unknown-unknown/release/hello-world.wasm demo/
	cp static/index.html demo/index.html
	cargo +nightly web start --target-webasm
