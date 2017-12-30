start:
	cargo +nightly web build --target-webasm

package:
	cargo +nightly package
