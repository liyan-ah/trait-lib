use std::sync::Arc;

pub trait Check {
    fn check(&self, arg: i32) -> bool;
}

struct TypeCheck {
    level: i32,
}

impl Check for TypeCheck {
    fn check(&self, arg: i32) -> bool {
        self.level > arg
    }
}

pub fn check_trait(check: Arc<Box<dyn Check>>, arg: i32) -> bool {
    check.check(arg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn as_check(check: Arc<Box<dyn Check>>) {
        assert!(check.check(11));
    }

    #[test]
    fn type_check() {
        let check: Arc<Box<dyn Check>> = Arc::new(Box::new(TypeCheck { level: 12 }));

        assert!(check.check(11));
        as_check(check);
    }
}
