use std::fmt;
use std::rc::Rc;

pub struct Callback<EVENT, MSG>(Rc<dyn FnMut(EVENT) -> MSG>);

impl<EVENT, F, MSG> From<F> for Callback<EVENT, MSG>
where
    F: FnMut(EVENT) -> MSG + 'static,
{
    fn from(f: F) -> Self {
        Callback(Rc::new(f))
    }
}

impl<EVENT, MSG> Callback<EVENT, MSG> {
    pub fn emit(&mut self, event: EVENT) -> MSG {
        (Rc::get_mut(&mut self.0).unwrap())(event)
    }
}

impl<EVENT, MSG> Clone for Callback<EVENT, MSG> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<EVENT, MSG> PartialEq for Callback<EVENT, MSG> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<IN, OUT> fmt::Debug for Callback<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "||{{..}}")
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
