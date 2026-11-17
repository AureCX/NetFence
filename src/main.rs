mod filter;
use std::env;
use crate::filter::check_if_allowed;
use std::fs;

static COMMANDS: &[(&str, fn(&str) -> i32)] = &[
    ("check", check_if_allowed),
    ("add", add_command),
    ("remove", remove_command),
    ("help", help_command),
    ("view", view_command)
];

fn uppercase_first(data: &str) -> String {
    // Uppercase first letter.
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value);
        }
    }
    result
}

fn add_command(command: &str) -> i32 {
    // Implementation for adding a command
    if command.is_empty() {
        println!("No domain provided to add.");
        return -1;
    }
    println!("Domain '{}' added successfully.", command);
    0
}

fn remove_command(command: &str) -> i32 {
    // Implementation for removing a command
    if command.is_empty() {
        println!("No domain provided to remove.");
        return -1;
    }
    println!("Domain '{}' removed successfully.", command);
    0
}

fn help_command(_command: &str) -> i32 {
    println!("Available commands:");
    println!("  check <domain>   - Check if the domain is allowed.");
    println!("  add <domain>     - Add a domain to the allowed list.");
    println!("  remove <domain>  - Remove a domain from the allowed list.");
    println!("  help          - Show this help message.");
    0
}

fn view_command(command: &str) -> i32 {
    let file = fs::File::open("rules.json")
        .expect("File should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("File should be proper JSON");
    let mut allowed_urls: Vec<String> = Vec::new();
    if let Some(rules) = json.as_array() {
        for rule in rules {
            if rule["action"] == command || rule["action"] == uppercase_first(command) {
                if let Some(url) = rule["pattern"].as_str() {
                    allowed_urls.push(url.to_string());
                }
            }
        }
    }
    println!("{} Domains:", uppercase_first(&format!("{}ed", command)));
    if allowed_urls.is_empty() {
        println!("  No allowed domains found.");
    } else {
        for url in allowed_urls {
            println!("  - {}", url);
        }
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
        } else if command == cmd && command == "help" {
            return _func("");
        } else if command == cmd &&args.len() <= 2 {
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
