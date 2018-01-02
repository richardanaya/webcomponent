# What is this?

A library for easily creating powerful new html elements in rust.

# Hello World

```rust
#[derive(Default)]
struct HelloWorld;

impl CustomElement for HelloWorld {
    fn created(&mut self, element:HtmlElement){
        element.set_text_content("Hello World!");
    }
}

...

HelloWorld::define("hello-world");
```

```html
<hello-world></hello-world>
```

[Demo](https://richardanaya.github.io/webcomponent-rs/examples/hello-world/demo/)
