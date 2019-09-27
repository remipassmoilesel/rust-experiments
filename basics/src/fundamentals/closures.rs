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

    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_millis(10));
        num * 2
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

    let mut lazy_1 = Lazy::new(expensive_closure);
    assert_eq!(lazy_1.get(2), 4);
    assert_eq!(lazy_1.get(2), 4);
    assert_eq!(lazy_1.get(6), 12);
    assert_eq!(lazy_1.get(6), 12);
}
