use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}

//
// LOG MESSENGER
// Example implementation of trait Messenger
// See below mock messenger
//

struct LogMessenger {}

impl LogMessenger {
    pub fn new() -> LogMessenger {
        LogMessenger {}
    }
}

impl Messenger for LogMessenger {
    fn send(&self, msg: &str) {
        println!("{:?}", msg)
    }
}

//
// LIMIT TRACKER
// Will use Messengers
//

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

//
// LIMIT TRACKER
// Will use messenger, and for tests we will use the mock messenger
//

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

fn tracker_example_usage() {
    let messenger = LogMessenger::new();
    let mut limit_tracker = LimitTracker::new(&messenger, 5);

    limit_tracker.set_value(1);
    limit_tracker.set_value(2);
    limit_tracker.set_value(3);
    limit_tracker.set_value(4);
    limit_tracker.set_value(5);
}

fn ref_cell_example() {
    // we can only borrow one mutable reference, borrow another will make program
    // panic **at runtime** instead at compile time:
    // thread 'main' panicked at 'already borrowed: BorrowMutError',

    let dumb_ref_cell = RefCell::new(5);
    let _ref_mut1 = dumb_ref_cell.borrow_mut();
    // let ref_mut2 = dumb_ref_cell.borrow_mut();
}

fn multiple_owner_of_mutable_data() {
    // Rc allow multiple value of immutable data
    // As RefCell seems immutable, we can share it throught Rc
    let value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    let c = Rc::clone(&value);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 1;
    *c.borrow_mut() += 1;

    println!("{:?}", value);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    let _ref_mut1 = a.borrow_mut();
    // let ref_mut2 = b.borrow_mut(); // will panic because we cannot hold two mutable references
}

pub fn main() {
    tracker_example_usage();
    ref_cell_example();
    multiple_owner_of_mutable_data();
}

#[cfg(test)]
mod tests {

    use super::*;

    //
    // MOCK MESSENGER
    // Using a RefCell for interior mutability
    //

    struct MockMessenger {
        // Interior Mutability pattern
        // here we must use a refcell, in order to get an immutable field we can mutate internally
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 5);

        limit_tracker.set_value(1);
        limit_tracker.set_value(2);
        limit_tracker.set_value(3);
        limit_tracker.set_value(4);

        let messages = mock_messenger.sent_messages.borrow();
        assert_eq!(messages.len(), 1);
        assert_eq!(
            messages.get(0),
            Some(&String::from(
                "Warning: You've used up over 75% of your quota!"
            ))
        );
    }
}
