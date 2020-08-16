use std::fmt;

pub struct Callback<EVENT>(Box<dyn FnMut(EVENT)>);

impl<EVENT, F> From<F> for Callback<EVENT>
where
    F: FnMut(EVENT) + 'static,
{
    fn from(f: F) -> Self {
        Callback(Box::new(f))
    }
}

impl<EVENT> Callback<EVENT>
where
    EVENT: PartialEq + Clone,
{
    pub fn emit(&mut self, event: EVENT) {
        (self.0)(event)
    }
}

impl<EVENT> fmt::Debug for Callback<EVENT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "||{{..}}")
    }
}

impl<EVENT> PartialEq for Callback<EVENT> {
    fn eq(&self, _rhs: &Self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cb_test2() {
        let mut e: i32 = 1;
        println!("initial e: {}", e);
        let mut cb = Callback::from(move |v: i32| {
            e += 1;
            println!("in callback e: {}", e);
        });
        cb.emit(5);

        println!("after callback e: {}", e);
    }
}
