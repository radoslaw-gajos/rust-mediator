use std::any::Any;

trait Widget {
    fn changed(&self, director: &Box<dyn Director>);
    fn as_any(&self) -> &dyn Any;
}

struct RadioWidget {
    value: bool,
}

impl Widget for RadioWidget {
    fn changed(&self, director: &Box<dyn Director>) {
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RadioWidget {
    fn new() -> Box<dyn Widget> {
        Box::new(RadioWidget {
            value: false,
        })
    }
}

trait Director {
    fn widget_changed(&self, widget: &str);
}

struct FoodDirector {
    radio: Box<dyn Widget>,
}

impl Director for FoodDirector {
    fn widget_changed(&self, widget: &str) {
        match widget {
            "radio" => {
                println!("Radio changed");
                let radio = self.radio.as_any()
                    .downcast_ref::<RadioWidget>()
                    .unwrap();

                println!("Radio value: {}", radio.value);

            },
            other => {
                println!("{other} changed");
            }
        }
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

        director.widget_changed("radio");
    }
}
