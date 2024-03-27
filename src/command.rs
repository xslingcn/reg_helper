use crate::{error::{RegResult, RegError}, register::register};

pub(crate) async fn handle_command(command: String) -> RegResult<String> {
    if command.starts_with("register ") || command.starts_with("r "){
        let args: Vec<&str> = command.split_whitespace().collect();
        if args.len() == 2 {
            let sln = args[1];
            println!("Registering SLN: {}", sln);
            return register(sln).await;
        }
    }
    if command.starts_with("reload ") {
    }
    Err(RegError::CommandNotFound(command))
}