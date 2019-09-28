use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn main() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    add_one_v1(2);
    add_one_v2(2);
    add_one_v3(2);
    add_one_v4(2);

    let expensive_multiply = |num| {
        println!("Multiplying by two, please wait a few minutes...");
        thread::sleep(Duration::from_millis(10));
        num * 2
    };

    let expensive_len = |x: String| {
        println!("Counting chars, please wait a few days...");
        thread::sleep(Duration::from_millis(10));
        x.len()
    };

    struct Lazy<T>
    where
        T: Fn(u32) -> u32,
    {
        func: T,
        values: HashMap<u32, u32>,
    }

    impl<T> Lazy<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(func: T) -> Lazy<T> {
            Lazy {
                func,
                values: HashMap::new(),
            }
        }

        fn get(&mut self, param: u32) -> u32 {
            match self.values.get(&param) {
                Some(x) => *x,
                None => {
                    let v = (self.func)(param);
                    self.values.insert(param, v);
                    v
                }
            }
        }
    }

    let mut lazy_1 = Lazy::new(expensive_multiply);
    assert_eq!(lazy_1.get(2), 4);
    assert_eq!(lazy_1.get(2), 4);
    assert_eq!(lazy_1.get(6), 12);
    assert_eq!(lazy_1.get(6), 12);

    let param = 12;
    let capture = |i| i + param;

    assert_eq!(capture(5), 17);

    // Force move of value in closures (can be useful for threads

    let param = vec![2, 4];
    let closure_that_do_not_take_ownership = |vec: Vec<i32>| -> Vec<i32> {
        vec.iter()
            .filter(|x| param.contains(x))
            .map(|x| *x)
            .collect()
    };

    closure_that_do_not_take_ownership(vec![1, 2, 3, 4, 5]);
    println!("{:?}", param);

    let param = vec![2, 4];
    let closure_that_take_ownership = move |vec: Vec<i32>| -> Vec<i32> {
        vec.iter()
            .filter(|x| param.contains(x))
            .map(|x| *x)
            .collect()
    };

    closure_that_take_ownership(vec![1, 2, 3, 4, 5]);
    println!("{:?}", param);  // will fail
}
