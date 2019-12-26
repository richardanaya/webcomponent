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
            let connect = create_callback_0(move || {
                el1.lock().connected();
            });
            let disconnect = create_callback_0(move || {
                el2.lock().disconnected();
            });
            let attribute_change = create_callback_3(move |name, old, new| {
                el3.lock().attribute_changed(name, old, new);
            });
            js!((e,a,b,c)=>{
              e.addHooks(a,b,c);
            })
            .invoke_4(JSNoDrop(element), connect, disconnect, attribute_change);
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


struct ShadowDom {
    fn_attach_shadow: JSInvoker,
    fn_set_shadow_html: JSInvoker,
    fn_set_html: JSInvoker,
}

impl Default for ShadowDom {
    fn default() -> Self {
        ShadowDom {
            fn_attach_shadow: js!((el,is_open)=>el.attachShadow({mode:is_open?"open":"closed"})),
            fn_set_shadow_html: js!((el,html)=>el.shadowRoot.innerHTML = html),
            fn_set_html: js!((el,html)=>el.innerHTML = html),
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
}
