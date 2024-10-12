use uuid::Uuid;
use std::sync::{Arc, Mutex};

trait Widget {
}

struct RadioWidget {
}

impl Widget for RadioWidget {
}

impl RadioWidget {
    fn new() -> Mutex<Box<dyn Widget>> {
        Mutex::new(Box::new(RadioWidget {
        }))
    }
}

trait WidgetRepository {
    fn add(&mut self, widget: Arc<Mutex<Box<dyn Widget>>>) -> Uuid;
    fn get(&self, uuid: Uuid) -> Arc<Mutex<Box<dyn Widget>>>;
    fn remove(&mut self, uuid: Uuid);
}

struct HMapWidgetRepository {
}

static WIDGET_REPOSITORY: Mutex<HMapWidgetRepository> = Mutex::new(HMapWidgetRepository {
});

impl WidgetRepository for HMapWidgetRepository {
    fn add(&mut self, widget: Arc<Mutex<Box<dyn Widget>>>) -> Uuid {
        Uuid::new_v4()
    }
    fn get(&self, uuid: Uuid) -> Arc<Mutex<Box<dyn Widget>>> {
        panic!("Not implemented yet");
    }
    fn remove(&mut self, uuid: Uuid) {
        panic!("Not implemented yet");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct MockWidget {
        id: String,
    }

    impl Widget for MockWidget {
    }

    impl MockWidget {
        fn new(id: String) -> Mutex<Box<dyn Widget>> {
            Mutex::new(Box::new(MockWidget {
                id
            }))
        }
    }

    #[test]
    fn should_add_and_get_widget() {
        // given
        let expected = Arc::new(MockWidget::new("widget_id".to_string()));

        // when
        let uuid = WIDGET_REPOSITORY.lock().unwrap().add(expected); 
        /*
        let actual: Box<MockWidget> = 
            (*WIDGET_REPOSITORY.lock().unwrap().get(uuid).lock().unwrap()).as_ref().downcast_ref::<MockWidget>();
            */
        let actual: Box<MockWidget> = 
            WIDGET_REPOSITORY.lock().unwrap().get(uuid).lock().unwrap().downcast_ref::<MockWidget>();

        // then
        //assert_eq!();
    }
}
