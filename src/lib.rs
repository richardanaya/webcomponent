#![recursion_limit="512"]
#[macro_use]
extern crate stdweb;

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

        function guid() {
          function s4() {
            return Math.floor((1 + Math.random()) * 0x10000)
              .toString(16)
              .substring(1);
          }
          return s4() + s4() + '-' + s4() + '-' + s4() + '-' +
            s4() + '-' + s4() + s4() + s4();
        }

        // create a generated custom element
        class GeneratedCustomElement extends HTMLElement {
          static get observedAttributes() {return (@{observed_attributes}).split(":"); }
          static get observedProperties() {return (@{observed_properties}).split(":"); }

          constructor() {
              super();
              this._id = guid();
              this._props = {};
              window.currentElement = this;
              (@{T::created})(this._id,this);
              window.currentElement = null;
              for(let i = 0 ; i < GeneratedCustomElement.observedProperties.length; i++) {
                  let name = GeneratedCustomElement.observedProperties[i];
                  Object.defineProperty(this, name, {
                      get() { return this._props[name]; },
                      set(newValue) {
                          let oldValue = this._props[name];
                          this._props[name] = newValue;
                          (@{T::property_changed})(this._id,this,name,this[name],newValue);
                      },
                      enumerable: true,
                      configurable: true
                    });
              }
          }

          connectedCallback() {
            window.currentElement = this;
            (@{T::connected})(this._id,this);
            window.currentElement = null;
          }

          disconnectedCallback() {
            window.currentElement = this;
            (@{T::disconnected})(this._id,this);
            window.currentElement = null;
          }

          attributeChangedCallback(attributeName, oldValue, newValue) {
            window.currentElement = this;
            (@{T::attribute_changed})(this._id,this,attributeName,oldValue,newValue);
            window.currentElement = null;
          }
        }

        // tell the dom to associate it with an html tag name
        customElements.define(@{tag_name}, GeneratedCustomElement);
    }
}

pub trait WebComponent {
    fn get_observable_attributes() -> Vec<&'static str> {vec![]}
    fn get_observable_properties() -> Vec<&'static str> {vec![]}
    fn get_property(element:&stdweb::web::HtmlElement,prop_name:String) -> stdweb::Value {
        js!(
             return @{element.as_ref()}[(@{String::from(prop_name)})];
         )
    }
    fn get_attribute(element:&stdweb::web::HtmlElement,attr_name:&str) -> Option<String> {
        js!(
             return @{element.as_ref()}.getAttribute(@{String::from(attr_name)});
         ).into_string()
    }
    fn set_inner_html(element:&stdweb::web::HtmlElement,text:&str) {
        js!{
            @{element.as_ref()}.innerHTML = @{text};
        };
    }
    fn created(_id:String,_element:stdweb::web::HtmlElement){}
    fn connected(_id:String,_element:stdweb::web::HtmlElement){}
    fn disconnected(_id:String,_element:stdweb::web::HtmlElement){}
    fn attribute_changed(_id:String,_element:stdweb::web::HtmlElement,_attribute_name:String,_old_value:stdweb::Value,_new_value:stdweb::Value){}
    fn property_changed(_id:String,_element:stdweb::web::HtmlElement,_attribute_name:String,_old_value:stdweb::Value,_new_value:stdweb::Value){}
}
