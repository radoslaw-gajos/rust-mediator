use crate::director::Director;

pub mod director {
    pub trait Director {
        /// notify director about the changes.
        /// Panics if name is invalid.
        fn widget_changed(&self, name: &str);
    }
}

pub mod food_director {
    use crate::director::Director;
    use crate::widget::Widget;
    use crate::radio_widget::RadioWidget;
    use crate::text_input_widget::TextInputWidget;

    use std::rc::Rc;
    use std::collections::HashMap;

    const LIKES_PIZZA: &str = "LIKES_PIZZA";
    const FAVOURITE_PIZZA: &str = "FAVOURITE_PIZZA";

    pub struct FoodDirector {
        likes_pizza: Rc<RadioWidget>,
        favourite_pizza: Rc<TextInputWidget>,
    }

    impl Director for FoodDirector {
        fn widget_changed(&self, name: &str) {
            match name {
                LIKES_PIZZA => {
                    let likes_pizza = *self.likes_pizza
                        .value
                        .borrow();
                    *self.favourite_pizza
                        .enabled
                        .borrow_mut() = likes_pizza;
                },
                FAVOURITE_PIZZA => (),
                other => panic!("{name} is an invalid name!"),
            }
        }
    }

    impl FoodDirector {
        fn new(likes_pizza: Rc<RadioWidget>, favourite_pizza: Rc<TextInputWidget>) 
            -> Rc<dyn Director> {
            let director: Rc<dyn Director> = Rc::new(FoodDirector {
                likes_pizza: Rc::clone(&likes_pizza),
                favourite_pizza: Rc::clone(&favourite_pizza),
            });
            likes_pizza.set_director(Rc::clone(&director));
            favourite_pizza.set_director(Rc::clone(&director));
            director
        }
    }

    fn likes_pizza_widget() -> Rc<RadioWidget> {
        Rc::new(RadioWidget::new(LIKES_PIZZA.to_string()))
    }

    fn favourite_pizza_widget() -> Rc<TextInputWidget> {
        Rc::new(TextInputWidget::new(FAVOURITE_PIZZA.to_string()))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_enable_favourite_pizza_when_likes_pizza() {
            // given
            let likes_pizza = likes_pizza_widget();
            let favourite_pizza = favourite_pizza_widget();
            let director = FoodDirector::new(
                Rc::clone(&likes_pizza), 
                Rc::clone(&favourite_pizza)
            );

            // when
            *likes_pizza.value.borrow_mut() = true;
            likes_pizza.changed();
            let fav_enabled = *favourite_pizza.enabled.borrow();

            // then
            assert!(fav_enabled);
        }
    }
}

pub mod widget {
    use crate::director::Director;

    use std::rc::Rc;

    pub trait Widget {
        /// informs director about the changes.
        /// expects set director. Panics if finds None.
        fn changed(&self);
        fn set_director(&self, director: Rc<dyn Director>);
    }
}

pub mod radio_widget {
    use crate::widget::Widget;
    use crate::director::Director;

    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct RadioWidget {
        name: String,
        director: RefCell<Option<Rc<dyn Director>>>,
        pub value: RefCell<bool>,
    }

    impl Widget for RadioWidget {
        fn changed(&self) {
            self.director
                .borrow()
                .as_ref()
                .expect("Director should be set")
                .widget_changed(&self.name);
        }
        fn set_director(&self, director: Rc<dyn Director>) {
            *self.director.borrow_mut() = Some(director);
        }
    }

    impl RadioWidget {
        pub fn new(name: String) -> RadioWidget {
            RadioWidget {
                name,
                director: RefCell::new(None),
                value: RefCell::new(false),
            }
        }
    }
}

pub mod text_input_widget {
    use crate::widget::Widget;
    use crate::director::Director;

    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct TextInputWidget {
        name: String,
        director: RefCell<Option<Rc<dyn Director>>>,
        pub value: RefCell<String>,
        pub enabled: RefCell<bool>,
    }

    impl Widget for TextInputWidget {
        fn changed(&self) {
            self.director
                .borrow()
                .as_ref()
                .expect("Director should be set")
                .widget_changed(&self.name);
        }
        fn set_director(&self, director: Rc<dyn Director>) {
            *self.director.borrow_mut() = Some(director);
        }
    }

    impl TextInputWidget {
        pub fn new(name: String) -> TextInputWidget {
            TextInputWidget {
                name,
                director: RefCell::new(None),
                value: RefCell::new("".to_string()),
                enabled: RefCell::new(false),
            }
        }
    }
}
