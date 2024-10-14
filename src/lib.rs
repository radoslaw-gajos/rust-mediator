use crate::director::Director;

pub mod director {
    pub trait Director {
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
    }

    impl Director for FoodDirector {
    }

    impl FoodDirector {
        fn new() -> Rc<dyn Director> {
            Rc::new(FoodDirector {
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct MockWidget;

        impl Widget for MockWidget {
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
        pub fn new() -> RadioWidget {
            RadioWidget {
                value: RefCell::new(false),
            }
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
        pub fn new() -> TextInputWidget {
            TextInputWidget {
                value: RefCell::new("".to_string()),
                enabled: RefCell::new(false),
            }
        }
    }
}
