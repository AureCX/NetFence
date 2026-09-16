pub fn check_for_match(input: &str) -> i32 {
    let blocked_domains = vec!["example.com", "testsite.org", "malicious.net"];
    for domain in blocked_domains {
        // Placeholder for actual matching logic
        if input.contains(domain) {
            return 1; // Match found
        }
    }
    return 0;
}