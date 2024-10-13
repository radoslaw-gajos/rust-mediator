use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::any::Any;

trait Director {
    // mediator methods
    fn widget_changed(&mut self, key: String);

    // helper methods
    fn attach(&mut self, key: String, widget: Box<dyn Widget>);
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

trait Widget {
    // mediator methods
    fn changed(&self);

    // helper methods
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

const LIKES_PIZZA: &str = "likes_pizza";
const FAVOURITE_PIZZA: &str = "favourite_pizza";

struct FoodDirector {
    widgets: HashMap<String, Box<dyn Widget>>,
}

impl Director for FoodDirector {
    // mediator methods
    fn widget_changed(&mut self, key: String) {
        match key.as_str() {
            LIKES_PIZZA => {
                let likes_pizza = self.widgets.get(LIKES_PIZZA)
                    .unwrap()
                    .as_any()
                    .downcast_ref::<RadioWidget>()
                    .unwrap()
                    .value;

                let favourite_pizza = self.widgets.get_mut(FAVOURITE_PIZZA)
                    .unwrap()
                    .as_mut_any()
                    .downcast_mut::<TextFieldWidget>()
                    .unwrap();

                favourite_pizza.enabled = likes_pizza;
                    
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
    fn as_mut_any(&mut self) -> &mut dyn Any {
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
    value: bool,
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
    // mediator methods
    fn changed(&self) {
        self.director
            .borrow_mut()
            .widget_changed(LIKES_PIZZA.to_string());
    }

    // helper methods
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl RadioWidget {
    fn new(director: Rc<RefCell<Box<dyn Director>>>) -> Box<dyn Widget> {
        Box::new(RadioWidget {
            director,
            value: false,
        })
    }
}

struct TextFieldWidget {
    director: Rc<RefCell<Box<dyn Director>>>,
    enabled: bool,
}


impl Widget for TextFieldWidget {
    // mediator methods
    fn changed(&self) {
    }

    // helper methods
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl TextFieldWidget {
    fn new(director: Rc<RefCell<Box<dyn Director>>>) -> Box<dyn Widget> {
        Box::new(TextFieldWidget {
            director,
            enabled: false,
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

    #[test]
    fn likes_pizza_should_enable_favourite_pizza() {
        // given
        let director = FoodDirector::new();
        let director = Rc::new(RefCell::new(director));
        let likes_pizza = RadioWidget::new(Rc::clone(&director));
        let favourite_pizza = TextFieldWidget::new(Rc::clone(&director));
        
        attach(&director, LIKES_PIZZA.to_string(), likes_pizza);
        attach(&director, FAVOURITE_PIZZA.to_string(), favourite_pizza);

        // when
        director.borrow_mut()
            .as_mut_any()
            .downcast_mut::<FoodDirector>()
            .unwrap()
            .widgets.get_mut(LIKES_PIZZA)
            .unwrap()
            .as_mut_any()
            .downcast_mut::<RadioWidget>()
            .unwrap().value = true;

        director.borrow_mut().widget_changed(LIKES_PIZZA.to_string());

        // TODO: probably I could make it work by turning widgets into Rc<RefCell's
        // for now I abandon this to stay sane
        /*
        {
            let mut likes_pizza_radio = director.borrow_mut()
                .as_mut_any()
                .downcast_mut::<FoodDirector>()
                .unwrap()
                .widgets.get_mut(LIKES_PIZZA)
                .unwrap()
                .as_mut_any()
                .downcast_mut::<RadioWidget>()
                .unwrap();

            likes_pizza_radio.changed();
        }
        */

        // then
        // TODO: refactor
        let textfield_enabled = director.borrow()
            .as_any()
            .downcast_ref::<FoodDirector>()
            .unwrap()
            .widgets.get(FAVOURITE_PIZZA)
            .unwrap()
            .as_any()
            .downcast_ref::<TextFieldWidget>()
            .unwrap()
            .enabled;

        assert!(textfield_enabled);
    }
}
