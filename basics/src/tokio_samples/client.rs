extern crate tokio;

use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

pub fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let client = TcpStream::connect(&addr)
        .and_then(|stream| {
            println!("created stream");

            // Process stream here.

            io::write_all(stream, "hello world\n").then(|result| {
                println!("wrote to stream; success={:?}", result.is_ok());
                Ok(())
            })
        })
        .map_err(|err| {
            // All tasks must have an `Error` type of `()`. This forces error
            // handling and helps avoid silencing failures.
            //
            // In our example, we are only going to log the error to STDOUT.
            println!("connection error = {:?}", err);
        });

    println!("About to create the stream and write to it...");
    tokio::run(client);
    println!("Stream has been created and written to.");
}
