use webcomponent::*;
use js_ffi::*;

struct HelloWorld(JSObject);

impl CustomElement for HelloWorld {
    fn new(element:JSObject) -> Self {
        HelloWorld(element)
    }
    fn connected(&mut self){
        set_property(&self.0,"innerHTML","Hello World!");
    }
}

#[no_mangle]
fn main() {
    HelloWorld::register("hello-world");
}