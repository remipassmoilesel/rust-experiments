extern crate httpserver;

use httpserver::start_server;

fn main() {
    start_server(String::from("127.0.0.1"), String::from("7878"))
}
