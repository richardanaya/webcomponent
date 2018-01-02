extern crate stdweb;
extern crate webcomponent;
use webcomponent::CustomElement;

struct HelloWorld;
impl CustomElement for HelloWorld {
    fn created(_id:String, element:stdweb::web::HtmlElement){
        Self::set_inner_html(&element,"<h1>Hello World!</h1>");
    }
}

fn main() {
    // get std wb started
    stdweb::initialize();

    // define the web components we will use
    HelloWorld::register("hello-world");

    // keep std event going
    stdweb::event_loop();
}
