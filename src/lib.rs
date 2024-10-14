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
}

pub mod widget {
    pub trait Widget {
    }
}

pub mod radio_widget {
    use crate::widget::Widget;

    use std::cell::RefCell;

    pub struct RadioWidget {
        value: RefCell<bool>,
    }

    impl Widget for RadioWidget {
    }
}

pub mod text_input_widget {
    use crate::widget::Widget;

    pub struct TextInputWidget {
        value: RefCell<String>,
        enabled: RefCell<bool>,
    }

    impl Widget for TextInputWidget {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
    }
}
