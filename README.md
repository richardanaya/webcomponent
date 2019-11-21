# webcomponent

<a href="https://docs.rs/webcomponent"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

[Web components](https://www.webcomponents.org/) are W3C standard for writing your own HTML element. `webcomponent` is a Rust library for easily writing your own web components in Rust with [`js_ffi`](https://github.com/richardanaya/js_ffi).

# Hello World
```toml
[dependencies]
webcomponent="0.1"
```
```rust
#[derive(Default)]
struct HelloWorld;

impl CustomElement for HelloWorld {
    fn created(&mut self, element:HtmlElement){
        js!().set_text_content("Hello World!");
    }
}

HelloWorld::define("hello-world");
```
```html
<!-- a polyfill for web components on some browsers -->
<script src="https://unpkg.com/@webcomponents/webcomponentsjs@latest/webcomponents-loader.js"></script>
<!-- for running your js_ffi library -->
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi/js_ffi.js"></script>
<!-- get things started -->
<script>js_ffi.run("helloworld.wasm");</script>
<!-- now you can put your hello-world element anywhere! -->
<hello-world></hello-world>
```
```makefile
# cli commands for building web assembly I find useful
build:
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/helloworld.wasm .
lint:
	@cargo fmt
serve:
	python3 -m http.server 8080
```


See demo [here](https://richardanaya.github.io/webcomponent/examples/hello-world/demo/)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `webcomponent` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

