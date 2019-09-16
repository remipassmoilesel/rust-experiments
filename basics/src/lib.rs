use fundamentals::*;
use testing::calculator::*;
use tokio_network::*;

pub mod fundamentals;
pub mod testing;
mod tokio_network;

pub fn main() {
    conditions::main();
    variables::main();
    loops::main();
    functions::main();
    operations::main();
    ownership::main();
    structs::main();
    enums::main();
    module_system::main();
    collections::main();

    client::main();

    Calculator::add(&1, &1);
}
