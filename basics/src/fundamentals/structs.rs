pub fn main() {
    // Struct definition

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    struct Value; // unit struct

    // Struct value

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Mutable struct value, all fields are mutable

    let mut mutableUser = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    mutableUser.email = String::from("anotheremail@example.com");

    // Struct builder

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Create struct from another struct

    fn copy_user(user: User, username: String, email: String) {
        User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user
        };
    }
    let user3 = copy_user(user1, String::from("user2"), String::from("user2@mail.com"));

    // Tuple structs

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Debug display

    #[derive(Debug)] // That annotation enable debugging display
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 25,
        height: 25,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 20,
    };
    println!("rect1 is {:?}", rect1); // one line output
    println!("rect1 is  {:#?}", rect1); // multiple lines output

    // Add methods on struct

    impl Rectangle {
        fn area(&self) -> u32 {
            // self can take ownership or borrow mutability too
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn is_bigger_than(&self, other: &Rectangle) -> bool {
            self.area() > other.area()
        }
    }

    println!("{}", rect1.area());
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.is_bigger_than(&rect2));

    // Associated functions (methods that does not have a self)

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    println!("{:?}", Rectangle::square(25));
}
