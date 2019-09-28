use std::ops::Deref;

pub fn main() {
    simple_box_sample();
    deref_operator();
    deref_trait_and_coercion();
    drop_trait();
}

fn simple_box_sample() {
    // box stores on the heap

    let stored_on_heap = Box::new(5);
    println!("stored_on_heap = {}", stored_on_heap);
}

fn deref_operator() {
    // deref operator: *

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Box implement Deref so we can use deref operator

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_trait_and_coercion() {
    // custom box implementation

    #[derive(Debug)]
    struct MyBox<T> {
        value: Box<T>,
    }

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox { value: Box::new(x) }
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            self.value.as_ref() // here we must return a reference to a value
        }
    }

    let x = MyBox::new(5);
    assert_eq!(5, *x);

    // deref coercion automatically deref function arguments

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // TODO: see DerefMut
}

pub fn drop_trait() {
    // Drop trait, executed when a value is going out of scope

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
