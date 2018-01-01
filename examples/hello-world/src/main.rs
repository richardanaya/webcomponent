extern crate stdweb;
extern crate webcomponent;

use webcomponent::{
    WebComponent,
    define,
    set_inner_html
};

#[derive(Default)]
struct HelloWorld;

impl WebComponent for HelloWorld {
    fn created(&mut self){
        set_inner_html("Hello World!");
    }
}

fn main() {
    // get std wb started
    stdweb::initialize();

    // define the web components we will use
    define::<HelloWorld>("hello-world");

    // keep std event going
    stdweb::event_loop();
}
