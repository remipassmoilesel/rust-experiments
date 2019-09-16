//
// Tests conventions:
//  https://doc.rust-lang.org/book/ch11-03-test-organization.html
//

#[cfg(test)]
mod tests {
    use crate::testing::calculator::Calculator;

    use super::*;

    #[test]
    fn add_should_work() {
        assert_eq!(Calculator::add(&2, &2), Ok(4));
    }

    #[test]
    #[should_panic(expected = "Expected panic message")]
    fn failing_test() {
        panic!("Expected panic message");
    }

    #[test]
    fn very_good_test() {
        assert!(true);
        assert_eq!(true, true);
        assert_ne!(true, false);
    }

    #[test]
    fn with_custom_messages() {
        assert_eq!(true, true, "True was not true");
    }

    #[test]
    #[ignore]
    fn ignoredTest() {
        assert_eq!(true, false, "True was not true");
    }
}
