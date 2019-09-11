pub fn main() {

    fn get_bigger(nb1: i32, nb2: i32) -> i32 {
        match nb1 {
            x if x > nb2 => {
                nb1
            }
            _ => {
                nb2
            }
        }
    }

    

}