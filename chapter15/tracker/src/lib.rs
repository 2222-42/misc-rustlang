pub trait Messanger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker <'a, T: 'a + Messanger> {
    messanger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where T: Messanger{
    pub fn new(messanger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker{
            messanger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messanger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messanger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messanger.send("Error: You are over your quota!");
        }
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessanger {
        sent_messages: RefCell<Vec<String>>,
    }
    
    impl MockMessanger {
        fn new() -> MockMessanger {
            MockMessanger{sent_messages: RefCell::new(vec![])}
        }
    }

    impl Messanger for MockMessanger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messanger = MockMessanger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messanger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messanger.sent_messages.borrow().len(), 1);
    }
}
