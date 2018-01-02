extern crate stdweb;
extern crate webcomponent;

use webcomponent::{
    WebComponent,
    define
};
use stdweb::web::{
    INode
};

#[derive(Default)]
struct HelloWorld;

impl WebComponent for HelloWorld {
    fn created(_id:String, element:stdweb::web::HtmlElement){
        element.set_text_content("Hello World!");
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
