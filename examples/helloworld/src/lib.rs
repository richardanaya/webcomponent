use webcomponent::*;
use js_ffi::*;

#[derive(Default)]
struct HelloWorld;

impl CustomElement for HelloWorld {
    fn created(&mut self, element:JSValue){
        js!((el,x)=>x.innerHTML=x;).invoke_2(element,"Hello World!");
    }
}

#[no_mangle]
fn main() {
    HelloWorld::register("hello-world");
}