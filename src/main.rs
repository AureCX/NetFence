mod filter;
use std::env;
use crate::filter::check_if_allowed;

static COMMANDS: &[(&str, fn(&str) -> i32)] = &[
    ("check", check_if_allowed),
    ("add", add_command),
    ("remove", remove_command)
];

fn add_command(command: &str) -> i32 {
    // Implementation for adding a command
    if command.is_empty() {
        println!("No command provided to add.");
        return -1;
    }
    0
}

fn remove_command(command: &str) -> i32 {
    // Implementation for removing a command
    if command.is_empty() {
        println!("No command provided to remove.");
        return -1;
    }
    0
}

fn get_line_command() -> i32 {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No command provided. Use --help for usage information.");
        return -1;
    }
    if args[1] == "--help" || args[1] == "-h" {
        println!("Help requested.");
        return -1;
    }
    let command = args[1].clone();
    for &(cmd, _func) in COMMANDS {
        if command == cmd && args.len() > 2 {
            return _func(args[2].as_str());
        } else if args.len() <= 2 {
            println!("No argument provided for command: {}", command);
            return -1;
        }
    }
    println!("Unknown command: {}", command);
    return -1;
}

fn main() {
    get_line_command();
    // let result= ;
    // match result {
    //     Ok(Action::Block) => {
    //         println!("Not allowed!");
    //     }
    //     Ok(Action::Allow) => {
    //         println!("Allowed!");
    //     }
    //     Err(error) => {
    //         eprintln!("Failed to load rules: {}", error);
    //     }
    // }
}
