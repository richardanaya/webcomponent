# What is this?

A library for easily creating powerful new html elements in rust.

# Hello World

```rust
#[derive(Default)]
struct HelloWorld;

impl WebComponent for HelloWorld {
    fn created(&mut self, element:stdweb::web::HtmlElement){
        element.set_text_content("Hello World!");
    }
}

...

define<HelloWorld>("hello-world");
```

```html
<hello-world></hello-world>
```

[Demo](https://richardanaya.github.io/webcomponent-rs/examples/hello-world/demo/)
