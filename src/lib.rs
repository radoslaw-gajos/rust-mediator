use std::cell::RefCell;
use std::rc::Rc;

trait Widget {
    fn changed(widget: Box<dyn Widget>) {
        self.director().borrow_mut().widget_changed(self);
    }
    fn director(widget: Box<dyn Widget>) -> Rc<RefCell<dyn Director>>;
}

trait Director {
    fn widget_changed(&mut self, widget: Box<dyn Widget>);
}

struct RadioWidget {
    value: bool,
    director: Rc<RefCell<dyn Director>>,
}

impl RadioWidget {
    fn set_value(&self, value: bool) {
        self.value = value;
        self.changed();
    }
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
