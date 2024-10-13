trait Widget {
    fn changed(&self, director: &Box<dyn Director>);
}

struct RadioWidget {
}

impl Widget for RadioWidget {
    fn changed(&self, director: &Box<dyn Director>) {
    }
}

impl RadioWidget {
    fn new() -> Box<dyn Widget> {
        Box::new(RadioWidget {})
    }
}

trait Director {
    fn widget_changed(&self, widget: &Box<dyn Widget>);
}

struct FoodDirector {
    radio: Box<dyn Widget>,
}

impl Director for FoodDirector {
    fn widget_changed(&self, widget: &Box<dyn Widget>) {
    }
}

impl FoodDirector {
    fn new(radio: Box<dyn Widget>) -> Box<dyn Director> {
        Box::new(FoodDirector {
            radio,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let radio: Box<dyn Widget> = RadioWidget::new();
        let director: Box<dyn Director> = FoodDirector::new(radio);

        //radio.changed(&director);
        //director.widget_changed(&radio);
    }
}
