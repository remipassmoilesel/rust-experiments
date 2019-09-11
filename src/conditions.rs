pub fn main() {
    let age: i32 = 17;

    if age >= 18 {
        println!("majeur !");
    } else {
        println!("mineur !");
    }

    let val = if age >= 18 {
        "majeur !"
    } else {
        "mineur !"
    };

    let my_string = "hello";

    match my_string {
        "bonjour" => {
            println!("français");
        }
        _ => {
            println!("je ne connais pas cette langue...");
        }
    }

    let value = match my_string {
        "bonjour" => {
            "français"
        }
        _ => {
            "je ne connais pas cette langue..."
        }
    };

    let age: i32 = 17;

    let age = match age {
        x if x > 15 && x > 18 => {
            "ado !"
        }
        _ => {
            "mineur !"
        }
    };
}