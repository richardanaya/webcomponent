# What is this?

I wanted to play around with [stdweb](https://github.com/koute/stdweb) to create a proof of concept to allow Web Components easily. If you aren't familiar with them. They are blocks of code that allow us to create our own html tags. Here's an example of where i'm going:

```rust
struct HelloWorld;

impl WebComponent for HelloWorld {
    fn get_element_name() -> &'static str {"hello-world"}

    fn constructor(){
      set_inner_html(r#"
          <style>
              hello-world button {
                  border: solid 1px black;
                  border-radius: 5px;
                  padding: 5px;
                  font-family: arial;
              }
          </style>
          <button>Hello World!</button>
        "#);
       add_event_listener("click",||{
           alert("Surprise!");
       })
    }
}

...

define_web_component(HelloWorld);
```

```html
<hello-world></hello-world>
```

Would output in the browser a button with "Hello World!" that alerts when clicked.

Here's a simple demo of this working: https://richardanaya.github.io/rust-webcomponent/

If you are familiar with chrome dev tools, try inspecting the elements and modify the "greeting" and "name" attributes.  For more details check out the source code of this project!

Overall I think I was pleased how easy this mostly was to do.  I would like a more rust native API for interacting with the DOM within the web component as a next step.

# How to run this

```bash
curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain=nightly
cargo +nightly web start --target-webasm
```
