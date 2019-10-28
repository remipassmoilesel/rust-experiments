pub fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    // let total2: i32 = v1_iter.sum(); // not allowed as sum() take ownership of iterator

    assert_eq!(total, 55);

    let param = 2;
    let v1_filtered: Vec<&i32> = v1.iter().filter(|x| *x % param == 0).collect();

    assert_eq!(v1_filtered, [&2, &4, &6, &8, &10]);

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32; // this is an associated type of trait Iterator

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None // To stop iterator, return None
            }
        }
    }

    let mut c = Counter::new(); // mutable reference as next() mutate counter state
    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);

    let v2: Vec<u32> = Counter::new().collect();

    assert_eq!(v2, [1, 2, 3, 4, 5]);

    let v2_filtered: Vec<u32> = Counter::new().filter(|x| x % 2 == 0).collect();

    assert_eq!(v2_filtered, [2, 4]);
}
