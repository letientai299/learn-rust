trait Messenger {
    fn send(&self, msg: &str);
}


struct LimitTracker<'a, T: Messenger> {
    mess: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    fn new(mess: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            mess,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.mess.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.mess.send("Warning: reach 90%!");
        } else if percentage_of_max > 0.75 {
            self.mess.send("Warning: reach 75%!");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMess {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMess {
        fn new() -> MockMess {
            MockMess {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMess {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_mess = MockMess::new();
        let mut tracker = LimitTracker::new(&mock_mess, 100);

        tracker.set_value(80);
        assert_eq!(mock_mess.sent_messages.borrow().len(), 1);
    }
}