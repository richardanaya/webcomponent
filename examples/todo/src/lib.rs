use webcomponent::*;
use web_console::*;
use js_ffi::*;

struct TodoList {
    element: HTMLElement,
    shadow_root:HTMLElement,
}

impl CustomElement for TodoList {
    fn new(element: HTMLElement) -> Self {
        TodoList {
            shadow_root: attach_shadow(&element, true),
            element,
        }
    }
    fn connected(&mut self) {
        self.render();
    }
}

impl TodoList {
    fn render(&mut self){
        set_html(&self.shadow_root, include_str!("todo-list.html"));
    }
}


struct TodoItem {
    element: HTMLElement,
    shadow_root:HTMLElement,
    is_done: bool,
}

impl CustomElement for TodoItem {
    fn new(element: HTMLElement) -> Self {
        let shadow = attach_shadow(&element, true);
        TodoItem{
            element: element,
            is_done:false,
            shadow_root: shadow,
        }
    }

    fn connected(&mut self) {
        set_html(&self.shadow_root, include_str!("todo-item.html"));
        js!(Node.prototype.addEventListener).call_2(
            &self.shadow_root,
            "click",
            create_callback_0(|| {
                js!(window.alert).invoke_1("I was clicked!");
            }),
        );
        self.render();
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["done"]
    }

    fn attribute_changed(
        &mut self,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "done" {
            if let Some(value) = new_value {
                if value == "yes" {
                    self.is_done = true;
                } else {
                    self.is_done = false;
                }
            } else {
                self.is_done = false;
            }
        }
        self.render();
    }
}

impl TodoItem {
    fn render(&mut self){
        if self.is_done {
            log("done");
        } else {
            log("not done");
        }
    }
}

#[no_mangle]
fn main() {
    TodoList::register("todo-list");
    TodoItem::register("todo-item");
}
