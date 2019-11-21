use webcomponent::*;
use js_ffi::*;

struct HelloWorld(JSObject);

impl CustomElement for HelloWorld {
    fn new(element:JSObject) -> Self {
        HelloWorld(element)
    }
    fn connected(&mut self){
        js!((el,x)=>el.innerHTML=x).invoke_2(&self.0,"Hello World!");
    }
}

#[no_mangle]
fn main() {
    HelloWorld::register("hello-world");
}