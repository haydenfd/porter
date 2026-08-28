mod cli;

use cli::Command;
use porter_core::ports;

fn main() {
    let command = cli::parse_command();

    match command {
        Ok(Command::List) => {
            println!("Printing all procs");
        }
        Ok(Command::Port(port)) => {
            println!("Inspecting {port}")
        }

        Err(error) => {
            println!("Error: {error}")
        }
    }
}
