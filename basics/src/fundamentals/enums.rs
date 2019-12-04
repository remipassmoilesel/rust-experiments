pub fn main() {
    enum Basic {
        ValueA,
        ValueB,
    }

    let _value_a = Basic::ValueA;
    let _value_b = Basic::ValueB;

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
        Move { x: i32, y: i32 },
        ChangeColor(i32, i32, i32),
    }

    // we can define methods on enums too
    impl Message {
        fn call(&self) {
            match self {
                Message::Move { x, y } => println!("Moving: {} {}", x, y),
                Message::ChangeColor(r, g, b) => println!("Changing color: {} {} {}", r, g, b),
            }
        }
    }

    let message1 = Message::Move { x: 1, y: 2 };
    message1.call();
    let message2 = Message::ChangeColor(255, 255, 255);
    message2.call();

    // Destructuring in let

    pub enum ShellError {
        GenericError { message: String },
    }

    let error = ShellError::GenericError { message: String::from_str("Error message") };
    let ShellError::GenericError { message } = error;
}
