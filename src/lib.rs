use std::cell::RefCell;
use std::rc::Rc;

trait Widget {
    fn changed(&self) {
        let widget: Box<&dyn Widget> = Box::new(self);
        self.director().borrow_mut().widget_changed(widget);
    }
    fn director(&self) -> Rc<RefCell<dyn Director>>;
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
