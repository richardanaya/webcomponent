use js_ffi::*;
use webcomponent::*;

struct HelloPerson(JSObject);

impl CustomElement for HelloPerson {
    fn new(element: JSObject) -> Self {
        HelloPerson(element)
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["first_name"]
    }

    fn connected(&mut self) {
        self.render();
    }

    fn attribute_changed(&mut self, _name: JSValue, _old_value: JSValue, _new_value: JSValue) {
        self.render();
    }
}

impl HelloPerson {
    fn render(&mut self){
        let first_name = get_attribute(&self.0, "first_name").unwrap_or("human".to_string());
        let msg = "Hello ".to_string() + &first_name;
        set_html(&self.0, &msg);
    }
}

#[no_mangle]
fn main() {
    HelloPerson::register("hello-person");
}
