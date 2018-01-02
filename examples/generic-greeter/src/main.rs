extern crate stdweb;
extern crate webcomponent;
use webcomponent::CustomElement;
use stdweb::web::{
    INode
};

struct GenericGreeter;

impl CustomElement for GenericGreeter {
    fn get_observable_attributes() -> Vec<&'static str> {vec!["greeting","name"]}

    fn created(_:String, element:stdweb::web::HtmlElement){
        Self::render(element);
    }

    fn attribute_changed(_:String, element:stdweb::web::HtmlElement, _:String, _:stdweb::Value, _:stdweb::Value){
        Self::render(element);
    }
}

impl GenericGreeter {
    fn render(element:stdweb::web::HtmlElement){
        let greeting = Self::get_attribute(&element,"greeting").unwrap_or(String::from("Hello"));
        let name = Self::get_attribute(&element,"name").unwrap_or(String::from("World"));
        element.set_text_content(&format!("{} {}! ",greeting,name));
    }
}

fn main() {
    // get std wb started
    stdweb::initialize();

    // define the web components we will use
    GenericGreeter::register("generic-greeter");

    // keep std event going
    stdweb::event_loop();
}
