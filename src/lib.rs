use crate::director::Director;

pub mod director {
    use crate::widget::Widget;

    use std::rc::Rc;

    pub trait Director {
        /// adds a widget for specific key
        fn attach(&self, key: String, widget: Rc<dyn Widget>);

        /// gets a widget based on the key.
        /// Panics if key is invalid.
        fn get(&self, key: String) -> Rc<dyn Widget>;
    }
}

pub mod food_director {
    use crate::director::Director;
    use crate::widget::Widget;
    use crate::radio_widget::RadioWidget;
    use crate::text_input_widget::TextInputWidget;

    use std::rc::Rc;
    use std::cell::RefCell;
    use std::collections::HashMap;

    pub struct FoodDirector {
        widgets: RefCell<HashMap<String, Rc<dyn Widget>>>,
    }

    impl Director for FoodDirector {
        fn attach(&self, key: String, widget: Rc<dyn Widget>) {
            self.widgets
                .borrow_mut()
                .insert(key, widget);
        }
        fn get(&self, key: String) -> Rc<dyn Widget> {
            match self.widgets
                .borrow()
                .get(&key) {
                    Some(widget) => Rc::clone(widget),
                    None => panic!("invalid key"),
            }
        }
    }

    const LIKES_PIZZA: &str = "LIKES_PIZZA";
    const FAVOURITE_PIZZA: &str = "FAVOURITE_PIZZA";

    impl FoodDirector {
        fn new() -> Rc<dyn Director> {
            Rc::new(FoodDirector {
                widgets: RefCell::new(HashMap::new()),
            })
        }

        fn add_likes_pizza(&self) {
            let likes_pizza = RadioWidget::new();
            self.attach(LIKES_PIZZA.to_string(), likes_pizza);
        }

        fn add_favourite_pizza(&self) {
            let favourite_pizza = TextInputWidget::new();
            self.attach(FAVOURITE_PIZZA.to_string(), favourite_pizza);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct MockWidget;

        impl Widget for MockWidget {
        }

        #[test]
        fn should_add_a_widget() {
            // given
            let director = FoodDirector::new();
            let widget: Rc<dyn Widget> = Rc::new(MockWidget);

            // when
            director.attach("mock".to_string(), Rc::clone(&widget));
            let result = director.get("mock".to_string());

            // then
            assert!(Rc::ptr_eq(&widget, &result));
        }

        #[test]
        fn should_enable_favourite_pizza_when_likes_pizza() {
            // given
            let director = FoodDirector::new();
        }
    }
}

pub mod widget {
    pub trait Widget {
    }
}

pub mod radio_widget {
    use crate::widget::Widget;

    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct RadioWidget {
        value: RefCell<bool>,
    }

    impl Widget for RadioWidget {
    }

    impl RadioWidget {
        pub fn new() -> Rc<dyn Widget> {
            Rc::new(RadioWidget {
                value: RefCell::new(false),
            })
        }
    }
}

pub mod text_input_widget {
    use crate::widget::Widget;

    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct TextInputWidget {
        value: RefCell<String>,
        enabled: RefCell<bool>,
    }

    impl Widget for TextInputWidget {
    }

    impl TextInputWidget {
        pub fn new() -> Rc<dyn Widget> {
            Rc::new(TextInputWidget {
                value: RefCell::new("".to_string()),
                enabled: RefCell::new(false),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
    }
}
