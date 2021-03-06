pub fn main() {
    let age: i32 = 17;

    if age >= 18 {
        println!("majeur !");
    } else {
        println!("mineur !");
    }

    let _val = if age >= 18 { "majeur !" } else { "mineur !" };

    let my_string = "hello";
    match my_string {
        "bonjour" => println!("français"),
        _ => println!("je ne connais pas cette langue..."),
    }

    let _value = match my_string {
        "bonjour" => "français",
        _ => "je ne connais pas cette langue...",
    };

    let age: i32 = 17;

    let _age = match age {
        x if x > 15 && x > 18 => "ado !",
        _ => "mineur !",
    };

    let maybe_number = Option::Some(50);
    let absent_number: Option<i32> = Option::None;
    assert_eq!(maybe_number.is_none(), false);
    assert_eq!(maybe_number.is_some(), true);
    assert_eq!(maybe_number.map(|value| value * 2), Some(100));

    fn option_plus_5(number: Option<i32>) -> Option<i32> {
        match number {
            Some(i) => Option::Some(i + 5),
            _ => None,
        }
    }

    let _five = option_plus_5(maybe_number);
    let _none = option_plus_5(absent_number);

    // If let can be shorter than match if only one arm is needed
    // Both solutions below are the same
    let y = Some(3);
    if let Some(3) = y {
        println!("{:?}", y)
    }

    match y {
        Some(3) => println!("{:?}", y),
        _ => (),
    }
}
