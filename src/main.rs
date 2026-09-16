mod filter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let result = filter::check_for_match(args[1].as_str());
    if result == 1 {
        println!("Not allowed!")
    } else {
        println!("Allowed!")
    }
}
