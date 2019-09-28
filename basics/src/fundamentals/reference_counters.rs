use std::rc::Rc;

/**
    Reference counters represent multiple ownership and allow to read data from multiple places
*/
pub fn main() {
    let value = Rc::new(vec![1, 2, 3, 4, 5, 6]);
    let value_ref = Rc::clone(&value); // similar as value.clone(), but convention is to use Rc::clone

    println!("References to value: {:?}", Rc::strong_count(&value)); // count how many reference to value

    let value_ref2 = Rc::clone(&value);

    println!("References to value: {:?}", Rc::strong_count(&value));

    {
        let value_ref3 = Rc::clone(&value);
        println!("References to value: {:?}", Rc::strong_count(&value));
    }

    println!(
        "One less due to previous scope end: {:?}",
        Rc::strong_count(&value)
    );
}
