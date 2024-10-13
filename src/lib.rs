use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::any::Any;

trait Director {
    fn attach(&mut self, key: String, widget: Box<dyn Widget>);
    fn as_any(&self) -> &dyn Any;
}

trait Widget {
}

struct FoodDirector {
    widgets: HashMap<String, Box<dyn Widget>>,
}

impl Director for FoodDirector {
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

struct RadioWidget {
    director: Rc<RefCell<Box<dyn Director>>>,
}

fn attach(
    director: Rc<RefCell<Box<dyn Director>>>,
    key: String,
    widget: Box<dyn Widget>) {
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
        director.borrow_mut()
            .attach(String::from("likes_pizza"), widget);

        // then
        let widget_count = director.borrow()
            .as_any()
            .downcast_ref::<FoodDirector>()
            .unwrap()
            .widgets.len();

        assert_eq!(widget_count, 1);
    }
}
