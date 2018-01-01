#![recursion_limit="512"]
#[macro_use]
extern crate stdweb;
use std::cell::RefCell;
use std::collections::HashMap;

// This function allows us to define a new html element hooked up to the static members of a type
pub fn define<T:WebComponent + 'static>(tag_name:&str) {

    // we need to use the array of attribute names we should be observing
    // and pass them in as a joined string since giving arrays to stdweb
    // isn't possible or expensive
    let observed_attributes = T::get_observable_attributes().join(":");
    let observed_properties = T::get_observable_properties().join(":");

    js! {
        // use a global variable that allows us to give a context for what element
        // is currently handling an event
        window.currentElement = null;

        // create a generated custom element
        class GeneratedCustomElement extends HTMLElement {
          static get observedAttributes() {return (@{observed_attributes}).split(":"); }
          static get observedProperties() {return (@{observed_properties}).split(":"); }

          constructor() {
              super();
              this._context = (@{T::create_context_identifier})(Math.random());
              window.currentElement = this;
              (@{T::created_with_context})(this._context);
              window.currentElement = null;
              for(let i = 0 ; i < GeneratedCustomElement.observedProperties.length; i++) {
                  let name = GeneratedCustomElement.observedProperties[i];
                  Object.defineProperty(this, name, {
                      get() { return (@{T::get_property})(this._context,name); },
                      set(newValue) { (@{T::property_changed_with_context})(this._context,name,this[name],newValue); },
                      enumerable: true,
                      configurable: true
                    });
              }
          }

          connectedCallback() {
            window.currentElement = this;
            (@{T::connected_with_context})(this._context);
            window.currentElement = null;
          }

          disconnectedCallback() {
            window.currentElement = this;
            (@{T::disconnected_with_context})(this._context);
            window.currentElement = null;
          }

          attributeChangedCallback(attributeName, oldValue, newValue) {
            window.currentElement = this;
            (@{T::attribute_changed_with_context})(this._context,attributeName,oldValue,newValue);
            window.currentElement = null;
          }
        }

        // tell the dom to associate it with an html tag name
        customElements.define(@{tag_name}, GeneratedCustomElement);
    }
}

pub fn log(msg:&str) {
    js! {
        console.log(@{msg});
    }
}

pub fn get_attribute(attr_name:&str) -> String{
    let result = js! {
        return window.currentElement.getAttribute(@{attr_name})||"";
    };
    result.as_str().unwrap().to_string()
}

pub fn alert(msg:&str) {
    js! {
        alert(@{msg});
    }
}

pub fn set_inner_html(html:&str){
    js! {
        window.currentElement.innerHTML = @{html};
    }
}

pub fn set_child_inner_html(target:&str,html:&str){
    js! {
        window.currentElement.querySelector(@{target}).innerHTML = @{html};
    }
}

pub fn add_event_listener(event_type:&str,handler:fn()->()){
    js! {
        window.currentElement.addEventListener(@{event_type}, () => {
            (@{handler})();
        })
    }
}

pub trait WebComponent : Default {
    fn get_or_create_context<T:Default>(stdweb::Value)-> T {
        T::default()
    }
    fn destroy_context(&mut self) {

    }
    fn create_context_identifier(random_seed:f64) -> stdweb::Value {
         stdweb::Value::from(random_seed)
    }
    fn get_observable_attributes() -> Vec<&'static str> {vec![]}
    fn get_observable_properties() -> Vec<&'static str> {vec![]}
    fn get_property(_context:stdweb::Value,_prop_name:String) -> stdweb::Value { stdweb::Value::Undefined }
    fn created_with_context(context:stdweb::Value){
        let mut t = Self::get_or_create_context::<Self>(context);
        t.created();
    }
    fn created(&mut self){}
    fn connected_with_context(context:stdweb::Value){
        let mut t = Self::get_or_create_context::<Self>(context);
        t.connected();
    }
    fn connected(&mut self){}
    fn disconnected_with_context(context:stdweb::Value){
        let mut t = Self::get_or_create_context::<Self>(context);
        t.disconnected();
    }
    fn disconnected(&mut self){
        // by default we don't know if it should be lingering if its removed from dom
        // so we assume removal means death
        self.destroy_context();
    }
    fn attribute_changed_with_context(context:stdweb::Value,attribute_name:String,old_value:stdweb::Value,new_value:stdweb::Value){
        let mut t = Self::get_or_create_context::<Self>(context);
        t.attribute_changed(attribute_name,old_value,new_value);
    }
    fn attribute_changed(&mut self,_attribute_name:String,_old_value:stdweb::Value,_new_value:stdweb::Value){}
    fn property_changed_with_context(context:stdweb::Value,attribute_name:String,old_value:stdweb::Value,new_value:stdweb::Value){
        let mut t = Self::get_or_create_context::<Self>(context);
        t.property_changed(attribute_name,old_value,new_value);
    }
    fn property_changed(&mut self,_attribute_name:String,_old_value:stdweb::Value,_new_value:stdweb::Value){}
}
