#![no_std]
use js_ffi::*;
#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

pub struct JSNoDrop(pub JSValue);

pub type HTMLElement = js_ffi::JSObject;

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

#[derive(Copy, Clone)]
struct Destructable {
    function: Option<JSFunction>,
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

            let destruct_connect = Arc::new(Mutex::new(Destructable { function: None }));
            let connect = create_callback_0(move || {
                el1.lock().connected();
            });
            destruct_connect.lock().function = Some(connect);

            let destruct_attribute_change = Arc::new(Mutex::new(Destructable { function: None }));
            let attribute_change = create_callback_3(move |name_obj, old_obj, new_obj| {
                let name = name_obj.as_string();
                let old = if old_obj.is_null() {
                    None
                } else {
                    Some(old_obj.as_string())
                };
                let new = if new_obj.is_null() {
                    None
                } else {
                    Some(new_obj.as_string())
                };
                el3.lock().attribute_changed(name, old, new);
            });
            destruct_attribute_change.lock().function = Some(connect);

            let destruct_disconnect = Arc::new(Mutex::new(Destructable { function: None }));
            let destruct_disconnect2 = destruct_disconnect.clone();
            let disconnect = create_callback_0(move || {
                el2.lock().disconnected();
                remove_callback(destruct_connect.lock().function.unwrap());
                remove_callback(destruct_disconnect.lock().function.unwrap());
                remove_callback(destruct_attribute_change.lock().function.unwrap());
            });
            destruct_disconnect2.lock().function = Some(disconnect);

            js!((e,a,b,c)=>{
              e.addHooks(a,b,c);
            })
            .invoke_4(JSNoDrop(element), connect, disconnect, attribute_change);
        });
        js!(
          (construct,elementName,attrNames)=>{
            let attrs = attrNames.split(",");
            class GeneratedCustomElement extends HTMLElement {
              constructor() {
                  super();
                  construct(this);
              }

              static get observedAttributes() {
                return attrs;
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
        .invoke_3(construct, name, &Self::observed_attributes().join(","));
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec![]
    }

    fn created(&mut self) {}
    fn connected(&mut self) {}
    fn disconnected(&mut self) {}
    fn attribute_changed(
        &mut self,
        _name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
    }
}

pub fn attach_shadow(el: impl ToJSValue, open: bool) {
    let shadow_dom = globals::get::<ShadowDom>();
    shadow_dom.attach_shadow(el, open);
}

pub fn set_shadow_html(el: impl ToJSValue, html: &str) {
    let shadow_dom = globals::get::<ShadowDom>();
    shadow_dom.set_shadow_html(el, html);
}

pub fn set_html(el: impl ToJSValue, html: &str) {
    let shadow_dom = globals::get::<ShadowDom>();
    shadow_dom.set_html(el, html);
}

pub fn get_attribute(el: impl ToJSValue, name: &str) -> Option<String> {
    let shadow_dom = globals::get::<ShadowDom>();
    shadow_dom.get_attribute(el, name)
}

struct ShadowDom {
    fn_attach_shadow: JSInvoker,
    fn_set_shadow_html: JSInvoker,
    fn_set_html: JSInvoker,
    fn_get_attribute: JSInvoker,
}

impl Default for ShadowDom {
    fn default() -> Self {
        ShadowDom {
            fn_attach_shadow: js!((el,is_open)=>el.attachShadow({mode:is_open?"open":"closed"})),
            fn_set_shadow_html: js!((el,html)=>el.shadowRoot.innerHTML = html),
            fn_set_html: js!((el,html)=>{
                debugger;
                el.innerHTML = html
            }),
            fn_get_attribute: js!((el,name)=>el.getAttribute(name)),
        }
    }
}

impl ShadowDom {
    pub fn attach_shadow(&self, el: impl ToJSValue, is_open: bool) {
        self.fn_attach_shadow.invoke_2(el, is_open);
    }

    pub fn set_shadow_html(&self, el: impl ToJSValue, html: &str) {
        self.fn_set_shadow_html.invoke_2(el, html);
    }

    pub fn set_html(&self, el: impl ToJSValue, html: &str) {
        self.fn_set_html.invoke_2(el, html);
    }

    pub fn get_attribute(&self, el: impl ToJSValue, name: &str) -> Option<String> {
        let result = self.fn_get_attribute.invoke_2(el, name);
        if result.is_null() {
            None
        } else {
            Some(result.as_string())
        }
    }
}
