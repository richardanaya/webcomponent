use js_ffi::*;

pub trait CustomElement {
    fn new(element:JSValue) -> Self;
    fn register(_name:&str){

    }
    fn created(&mut self){}
    fn connected(&mut self){}
    fn disconnected(&mut self){}
    fn attribute_changed(&mut self,_name:JSValue,_old_value:JSValue,_new_value:JSValue){}
}
