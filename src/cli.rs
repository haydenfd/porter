// cli parsing
use std::env;

pub enum Command {
    List,
    Port(u16),
}

pub fn parse_command() -> Result<Command, String> {
    let cli_args: Vec<String> = env::args().collect();

    if let Some(flag) = cli_args.get(1) {
        if flag == "ls" {
            return Ok(Command::List);
        }

        let port_val = flag
            .parse::<u16>()
            .map_err(|_| "Not a valid port number".to_string())?;

        return Ok(Command::Port(port_val));
    }

    Err("Expected `ls` or a port number".to_string())
}
