use webcomponent::*;
use js_ffi::*;

struct HelloWorld(JSValue);

impl CustomElement for HelloWorld {
    fn new(element:JSValue) -> Self {
        HelloWorld(element)
    }
    fn created(&mut self){
        js!((el,x)=>x.innerHTML=x;).invoke_2(self.0,"Hello World!");
    }
}

#[no_mangle]
fn main() {
    HelloWorld::register("hello-world");
}