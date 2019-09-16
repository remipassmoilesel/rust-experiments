use crate::module_system::front_of_house::hosting::add_to_waitlist;

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

pub fn main() {

    // Module system components:
    //  - Packages: A Cargo feature that lets you build, test, and share crates
    //  - Crates: A tree of modules that produces a library or executable
    //  - Modules and use: Let you control the organization, scope, and privacy of paths
    //  - Paths: A way of naming an item, such as a struct, function, or module

    // Every files placed in src/bin become a separated binary

    add_to_waitlist();

//
//    Use several objects with the same name:
//
//        use std::fmt::Result as FmtResult;
//        use std::io::Result as IoResult;
//
//        fn function1() -> FmtResult {
//            // --snip--
//        }
//
//        fn function2() -> IoResult<()> {
//            // --snip--
//        }
//
//    Or:
//
//        use std::fmt;
//        use std::io;
//
//        fn function1() -> fmt::Result {
//            // --snip--
//        }
//
//        fn function2() -> io::Result<()> {
//            // --snip--
//        }
//
//
//        Nested paths, that:
//
//        use std::{cmp::Ordering, io};
//        use std::io::{self, Write};
//
//        ...is equivalent to that
//
//        use std::cmp::Ordering;
//        use std::io;
//
//        Glob operators:
//        use std::collections::*;
//
}
