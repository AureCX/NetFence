mod filter;
use std::env;
use crate::filter::Action;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let result = filter::check_for_match(args[1].as_str());
    match result {
        Ok(Action::Block) => {
            println!("Not allowed!");
        }
        Ok(Action::Allow) => {
            println!("Allowed!");
        }
        Err(error) => {
            eprintln!("Failed to load rules: {}", error);
        }
    }
}
