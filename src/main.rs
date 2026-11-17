mod filter;
mod helper;
use std::env;
use crate::helper::uppercase_first;
use crate::filter::{check_if_allowed, parse_rules_json, Action, Rule};
use std::{fs, process::ExitCode};

static COMMANDS: &[(&str, fn(&str) -> u8)] = &[
    ("check", check_if_allowed),
    ("add", add_command),
    ("remove", remove_command),
    ("help", help_command),
    ("view", view_command)
];

fn add_command(command: &str) -> u8 {
    // Implementation for adding a command
    if command.is_empty() {
        println!("No domain provided to add.");
        return 1;
    }
    // Load existing rules
    let mut rules = match parse_rules_json("rules.json") {
        Ok(rules) => rules,
        Err(error) => {
            eprintln!("Failed to load rules: {}", error);
            return 1;
        }
    };
    // Avoid adding the same Allow rule twice
    if rules.iter().any(|rule|rule.pattern == command && rule.action == Action::Allow) {
        println!("Domain '{}' is already allowed.", command);
        return 0;
    }
    // Create and add the new rule
    let new_rule = Rule {
        pattern: command.to_string(),
        action: Action::Allow,
        priority: 200,
    };
    rules.push(new_rule);
    // Convert the updated rules back into JSON
    let json = match serde_json::to_string_pretty(&rules) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("Failed to serialize rules: {}", error);
            return 1;
        }
    };
    // Write the updated JSON to the file
    if let Err(error) = fs::write("rules.json", json) {
        eprintln!("Failed to save rules: {}", error);
        return 1;
    }
    println!("Domain '{}' added successfully.", command);
    0
}

fn remove_command(command: &str) -> u8 {
    // Implementation for removing a command
    if command.is_empty() {
        println!("No domain provided to remove.");
        return 1;
    }
    let mut rules = match parse_rules_json("rules.json") {
        Ok(rules) => rules,
        Err(error) => {
            eprintln!("Failed to load rules: {}", error);
            return 1;
        }
    };
    // Avoid adding the same Allow rule twice
    if rules.iter().any(|rule|rule.pattern == command && rule.action == Action::Allow) {
        rules.retain(|rule| !(rule.pattern == command && rule.action == Action::Allow));
        let json = match serde_json::to_string_pretty(&rules) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("Failed to serialize rules: {}", error);
            return 1;
         }
        };
        // Write the updated JSON to the file
        if let Err(error) = fs::write("rules.json", json) {
            eprintln!("Failed to save rules: {}", error);
            return 1;
        }
        println!("Domain '{}' removed successfully.", command);
        return 0;
    }
    // Convert the updated rules back into JSON
    println!("Domain '{}' not found in the allowed list.", command);
    0
}

fn help_command(_command: &str) -> u8 {
    println!("Available commands:");
    println!("  check <domain>   - Check if the domain is allowed.");
    println!("  add <domain>     - Add a domain to the allowed list.");
    println!("  remove <domain>  - Remove a domain from the allowed list.");
    println!("  view <action>    - View domains for a specific action (allow/block).");
    println!("  help          - Show this help message.");
    0
}

fn view_command(command: &str) -> u8 {
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
    if !allowed_urls.is_empty() {
        println!("{} Domains:", uppercase_first(&format!("{}ed", command)));
    }
    if allowed_urls.is_empty() {
        println!("  No domains found matching '{}', check ./NetFence --help for usage information.", command);
    } else {
        for url in allowed_urls {
            println!("  - {}", url);
        }
    }
    0
}

fn get_line_command() -> u8 {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No command provided. Use --help for usage information.");
        return 1;
    }
    if args[1] == "--help" || args[1] == "-h" {
        help_command("");
        return 0;
    }
    let command = args[1].clone();
    for &(cmd, _func) in COMMANDS {
        if command == cmd && args.len() > 2 {
            return _func(args[2].as_str());
        } else if command == cmd && command == "help" {
            return _func("");
        } else if command == cmd && args.len() <= 2 {
            println!("No argument provided for command: {}", command);
            return 1;
        }
    }
    println!("Unknown command: {}", command);
    return 1;
}

fn main() -> ExitCode {
    return ExitCode::from(get_line_command());
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
