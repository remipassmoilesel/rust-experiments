pub fn main() {
    enum Basic {
        ValueA,
        ValueB,
    }

    let valueA = Basic::ValueA;

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let four = IpAddr::V4(1, 2, 3, 4);
    let six = IpAddr::V6(String::from("a:b:c:d"));

    println!("{:?}", four);
    println!("{:?}", six);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // we can define methods on enums too
    impl Message {
        fn call(&self) {}
    }
}
