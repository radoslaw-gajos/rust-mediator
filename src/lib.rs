use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

trait Director {
    fn attach(&mut self, key: String, widget: Box<dyn Widget>);
}

trait Widget {
}

struct FoodDirector {
    widgets: HashMap<String, Box<dyn Widget>>,
}

impl Director for FoodDirector {
    fn attach(&mut self, key: String, widget: Box<dyn Widget>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let director = FoodDirector::new();
    }
}
