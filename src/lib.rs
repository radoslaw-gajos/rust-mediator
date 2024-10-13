use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

trait Widget {
    fn changed(&self, director: &Box<dyn Director>);
    fn as_any_mut(&self) -> &mut dyn Any;
}

struct RadioWidget {
    value: bool,
    //director: Option<Rc<Box<dyn Director>>>,
    director: Rc<RefCell<Option<Box<dyn Director>>>>,
}

impl Widget for RadioWidget {
    fn changed(&self, director: &Box<dyn Director>) {
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl RadioWidget {
    fn new() -> Box<dyn Widget> {
        Box::new(RadioWidget {
            value: false,
            director: Rc::new(RefCell::new(None)),
        })
    }
}

trait Director {
    fn widget_changed(&self, widget: &str);
    fn as_any(&self) -> &dyn Any;
}

struct FoodDirector {
    radio: Box<dyn Widget>,
}

fn radio_from_widget(widget: &mut Box<dyn Widget>) -> &mut RadioWidget {
    widget.as_any_mut()
        .downcast_mut::<RadioWidget>()
        .unwrap()
}

impl Director for FoodDirector {
    fn widget_changed(&mut self, widget: &str) {
        match widget {
            "radio" => {
                println!("Radio changed");
                let radio = radio_from_widget(&self.radio);
                println!("Radio value: {}", radio.value);

            },
            other => {
                println!("{other} changed");
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl FoodDirector {
    fn new(radio: Box<dyn Widget>) -> Box<dyn Director> {
        Box::new(FoodDirector {
            radio,
        })
    }
}

fn food_dir_from_widget(director: &Box<dyn Director>) -> &FoodDirector {
    director.as_any()
        .downcast_ref::<FoodDirector>()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let radio: Box<dyn Widget> = RadioWidget::new();
        let director: Box<dyn Director> = FoodDirector::new(radio);

        let food_dir = food_dir_from_widget(&director);
        let mut radio = radio_from_widget(&food_dir.radio);

        //let director = Rc::new(director);
        //radio.director = Some(Rc::clone(&director));
        //let radio_director = radio.director;
        //let radio_director = &radio.director;
        /*
        if let Some(rd) = *radio_director.borrow_mut() {
        }
        */
        //let radio_director = *radio_director.borrow_mut()
        //(*radio_director.borrow_mut().unwrap()) = *director;
        let rc_director = Rc::new(RefCell::new(Some(director)));
        //*radio_director.borrow_mut() = Some(director);
        radio.director = Rc::clone(&rc_director);

        director.widget_changed("radio");
    }
}
