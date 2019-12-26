use js_ffi::*;
use webcomponent::*;

struct HelloWorld(JSObject);

impl CustomElement for HelloWorld {
    fn new(element: JSObject) -> Self {
        HelloWorld(element)
    }
    fn connected(&mut self) {
        set_html(&self.0, "Hello World!");
    }
}

#[no_mangle]
fn main() {
    HelloWorld::register("hello-world");
}
