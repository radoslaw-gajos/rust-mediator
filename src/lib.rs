use std::cell::RefCell;
use std::rc::Rc;

trait Widget {
    fn Changed(&self) {
    }
    fn director(&self) -> Rc<RefCell<dyn Director>>;
}

trait Director {
    fn WidgetChanged(&mut self, widget: &Box<dyn Widget>);
}

struct RadioWidget {
    value: bool,
    director: Rc<RefCell<dyn Director>>,
}

struct FoodDirector {
    likes_pizza: RadioWidget,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDirector {
    }

    #[test]
    fn setting_value_should_call_director() {
    }
}
