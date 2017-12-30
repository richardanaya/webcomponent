#![recursion_limit="256"]
#[macro_use]
extern crate stdweb;

// This function allows us to define a new html element hooked up to the static members of a type
pub fn define<T:WebComponent + 'static>(tag_name:&str,_:T) {

    // we need to use the array of attribute names we should be observing
    // and pass them in as a joined string since giving arrays to stdweb
    // isn't possible or expensive
    let observed_attributes = T::get_observable_attributes().join(":");

    js! {
        // use a global variable that allows us to give a context for what element
        // is currently handling an event
        window.currentElement = null;

        // create a generated custom element
        class GeneratedCustomElement extends HTMLElement {
          static get observedAttributes() {return (@{observed_attributes}).split(":"); }

          constructor() {
              super();
              window.currentElement = this;
              (@{T::constructor})();
              window.currentElement = null;
          }

          connectedCallback() {
            window.currentElement = this;
            (@{T::connected})();
            window.currentElement = null;
          }

          disconnectedCallback() {
            window.currentElement = this;
            (@{T::disconnected})();
            window.currentElement = null;
          }

          attributeChangedCallback(attributeName, oldValue, newValue) {
            window.currentElement = this;
            (@{T::attribute_changed})(attributeName,oldValue||"",newValue||"");
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

pub trait WebComponent {
    fn get_observable_attributes() -> Vec<&'static str> {vec![]}
    fn constructor(){}
    fn connected(){}
    fn disconnected(){}
    fn attribute_changed(_attribute_name:String,_old_value:String,_new_value:String){}
}
