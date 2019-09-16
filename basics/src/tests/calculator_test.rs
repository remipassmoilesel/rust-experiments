
#[cfg(test)]
mod tests {
    use crate::tests::calculator::Calculator;

    #[test]
    fn add_should_work() {
        assert_eq!(Calculator::add(&2, &2), 4);
    }
}
