extern crate stdweb;
extern crate webcomponent;

use webcomponent::{
    WebComponent,
    define,
    set_inner_html
};

struct HelloWorld;

impl WebComponent for HelloWorld {
    fn constructor(){
        set_inner_html("Hello World!");
    }
}

fn main() {
    // get std wb started
    stdweb::initialize();

    // define the web components we will use
    define("hello-world",HelloWorld);

    // keep std event going
    stdweb::event_loop();
}
