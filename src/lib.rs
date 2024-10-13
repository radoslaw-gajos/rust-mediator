trait Director {
}

trait Widget {
}

struct FoodDirector {
}

impl Director for FoodDirector {
}

struct RadioWidget {
}

impl Widget for RadioWidget {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
    }
}
