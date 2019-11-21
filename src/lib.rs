use js_ffi::*;

pub trait CustomElement {
    fn register(name:&str){
      
    }
    fn created(&mut self,_element:JSValue){}
    fn connected(&mut self,_element:JSValue){}
    fn disconnected(&mut self,_element:JSValue){}
    fn attribute_changed(&mut self,_element:JSValue,_name:JSValue,_old_value:JSValue,_new_value:JSValue){}
}
