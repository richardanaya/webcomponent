use js_ffi::*;
use webcomponent::*;

struct HelloWorld(JSObject);

impl CustomElement for HelloWorld {
    fn new(element: JSObject) -> Self {
        HelloWorld(element)
    }
    fn connected(&mut self) {
        attach_shadow(&self.0, true);
        set_shadow_html(&self.0, "<div>Hello <slot name=\"fname\"></slot>!</div>");
        set_property(&self.0, "innerHTML", "<span slot=\"fname\">Richard</span>");
    }
}

#[no_mangle]
fn main() {
    HelloWorld::register("hello-world");
}
