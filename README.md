# What is this?

A library for easily creating powerful new html elements in rust.

# Hello World

```rust
struct HelloWorld;

impl WebComponent for HelloWorld {
    fn constructor(){
        set_inner_html("Hello World!");
    }
}

...

define("hello-world",HelloWorld);
```

```html
<hello-world></hello-world>
```

[Demo](https://richardanaya.github.io/webcomponent.rs/examples/hello-world/demo/)
