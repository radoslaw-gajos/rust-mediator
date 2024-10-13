use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::any::Any;

trait Director {
    // mediator methods
    fn widget_changed(&self, key: String);

    // helper methods
    fn attach(&mut self, key: String, widget: Box<dyn Widget>);
    fn as_any(&self) -> &dyn Any;
}

trait Widget {
}

const LIKES_PIZZA: &str = "likes_pizza";
const FAVOURITE_PIZZA: &str = "favourite_pizza";

struct FoodDirector {
    widgets: HashMap<String, Box<dyn Widget>>,
}

impl Director for FoodDirector {
    // mediator methods
    fn widget_changed(&self, key: String) {
        match key.as_str() {
            LIKES_PIZZA => {
            },
            other => {
                panic!("{other} key is not a valid key!");
            }
        }
    }

    // helper methods
    fn attach(&mut self, key: String, widget: Box<dyn Widget>) {
        self.widgets.insert(key, widget);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl FoodDirector {
    fn new() -> Box<dyn Director> {
        Box::new(FoodDirector {
            widgets: HashMap::new(),
        })
    }
}

// widgets
struct RadioWidget {
    director: Rc<RefCell<Box<dyn Director>>>,
}

fn attach(
    director: &Rc<RefCell<Box<dyn Director>>>,
    key: String,
    widget: Box<dyn Widget>) {
        director.borrow_mut()
            .attach(key, widget);
}

impl Widget for RadioWidget {
}

impl RadioWidget {
    fn new(director: Rc<RefCell<Box<dyn Director>>>) -> Box<dyn Widget> {
        Box::new(RadioWidget {
            director,
        })
    }
}

struct TextFieldWidget {
    director: Rc<RefCell<Box<dyn Director>>>,
}


impl Widget for TextFieldWidget {
}

impl TextFieldWidget {
    fn new(director: Rc<RefCell<Box<dyn Director>>>) -> Box<dyn Widget> {
        Box::new(TextFieldWidget {
            director,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_attach_widget() {
        // given
        let director = FoodDirector::new();
        let director = Rc::new(RefCell::new(director));
        let widget = RadioWidget::new(Rc::clone(&director));

        // when
        attach(&director, String::from("likes_pizza"), widget);

        // then
        let widget_count = director.borrow()
            .as_any()
            .downcast_ref::<FoodDirector>()
            .unwrap()
            .widgets.len();

        assert_eq!(widget_count, 1);
    }
}
