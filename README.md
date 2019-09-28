# Rust experiments

Reading:
- https://doc.rust-lang.org/book

See: 
- https://doc.rust-lang.org/book/
- https://doc.rust-lang.org/stable/rust-by-example/
- https://doc.rust-lang.org/stable/std/
- https://blog.guillaume-gomez.fr/Rust/
- https://www.youtube.com/watch?v=vOMJlQ5B-M0&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL

Todo:
- https://doc.rust-lang.org/book/ch15-06-reference-cycles.html


## Miscellaneous commands

Init a directory:

    $ cargo init


Create a package:

    $ cargo new path/to/package    


Add a dependency:

    $ cargo install cargo-watch


Build:

    $ cago build
    $ cago build --release
    

Debug:

    $ export RUST_BACKTRACE=1
    $ cargo run
    

Test:

    $ cargo test 
    $ cargo test -- --nocapture # do not capture prinln!() output


Reformat:

    $ cargo fmt
    

Update Rust:

    $ rustup update


Fix:

    $ cargo fix
