use js_ffi::*;
extern crate alloc;
use alloc::sync::Arc;
use spin::Mutex;

pub struct JSNoDrop(pub JSValue);

impl ToJSValue for JSNoDrop {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_OBJECT
    }
}

pub trait CustomElement {
    fn new(element: JSObject) -> Self
    where
        Self: core::marker::Sized + core::marker::Sync + core::marker::Send + 'static;
    fn register(name: &str)
    where
        Self: core::marker::Sized + core::marker::Sync + core::marker::Send + 'static,
    {
        let construct = create_callback_1(|element| {
            let el = Arc::new(Mutex::new(Self::new(JSObject(element))));
            let el1 = el.clone();
            let el2 = el.clone();
            let el3 = el.clone();
            let connect = create_callback_0(move ||{
              el1.lock().connected();
            });
            let disconnect = create_callback_0(move ||{
              el2.lock().disconnected();
            });
            let attribute_change = create_callback_3(move |name,old,new|{
              el3.lock().attribute_changed(name,old,new);
            });
            js!((e,a,b,c)=>{
              e.addHooks(a,b,c);
            }).invoke_4(JSNoDrop(element),connect,disconnect,attribute_change);
        });
        js!(
          (construct,elementName)=>{
            class GeneratedCustomElement extends HTMLElement {
              constructor() {
                  super();
                  construct(this);
              }

              connectedCallback() {
                self.connect();
              }

              disconnectedCallback() {
                self.disconnect();
              }

              attributeChangedCallback(attributeName, oldValue, newValue) {
                self.attributeChange(attributeName,oldValue,newValue)
              }

              addHooks(connect,disconnect,attributeChange){
                self.connect = connect;
                self.disconnect = disconnect;
                self.attributeChange = attributeChange;
              }
            }

            // tell the dom to associate it with an html tag name
            customElements.define(elementName, GeneratedCustomElement);
          }
        )
        .invoke_2(construct, name);
    }
    fn created(&mut self) {}
    fn connected(&mut self) {}
    fn disconnected(&mut self) {}
    fn attribute_changed(&mut self, _name: JSValue, _old_value: JSValue, _new_value: JSValue) {}
}
