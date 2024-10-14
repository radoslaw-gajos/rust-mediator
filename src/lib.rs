use crate::director::Director;

pub mod director {
    pub trait Director {
    }
}

pub mod food_director {
    use crate::director::Director;
    use crate::widget::Widget;

    use std::rc::Rc;
    use std::cell::RefCell;
    use std::collections::HashMap;

    pub struct FoodDirector {
        widgets: RefCell<HashMap<String, Rc<dyn Widget>>>,
    }

    impl Director for FoodDirector {
    }

    impl FoodDirector {
        pub fn new() -> Rc<dyn Director> {
            Rc::new(FoodDirector {
                widgets: RefCell::new(HashMap::new()),
            })
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
