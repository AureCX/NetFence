use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Action {
    Allow,
    Block,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub pattern: String,
    pub action: Action,
    pub priority: u32,
}

pub fn matches_url(pattern: &str, domain: &str) -> bool {
    // wildcard matching
    let mut ipat: usize = 0;  // Index for both input &str's in upper loop
    let mut idom: usize;      // Index for tame &str, used in lower loop
    let mut ipat_sequence: usize; // Index for prospective match after '*'
    let mut idom_sequence: usize; // Index for prospective match in tame &str
 
    // Find a first wildcard, if one exists, and the beginning of any  
    // prospectively matching sequence after it.
    loop
    {
        // Check for the end from the start.  Get out fast, if possible.
        if domain.len() <= ipat {
            if pattern.len() > ipat {
                while pattern.as_bytes()[ipat] == '*' as u8 {
                    ipat += 1; 
                    if pattern.len() <= ipat{
                        return true;       // "ab" matches "ab*".
                    }
                }
                return false;              // "abcd" doesn't match "abc".
            } else {
                return true;               // "abc" matches "abc".
            }
        } else if pattern.len() <= ipat {
            return false;                  // "abc" doesn't match "abcd".
        } else if pattern.as_bytes()[ipat] == '*' as u8 {
            // Got wild: set up for the second loop and skip on down there.
            idom = ipat;
            loop {
                ipat += 1;
                if pattern.len() <= ipat {
                    return true;               // "abc*" matches "abcd".
                } if pattern.as_bytes()[ipat] == '*' as u8 {
                    continue;
                } 
                break;
            }
            // Search for the next prospective match.
            if pattern.as_bytes()[ipat] != '?' as u8 {
                while pattern.as_bytes()[ipat] != domain.as_bytes()[idom] {
                    idom += 1;
                    if domain.len() <= idom {
                        return false;      // "a*bc" doesn't match "ab".
                    }
                }
            }
            // Keep fallback positions for retry in case of incomplete match.
            ipat_sequence = ipat;
            idom_sequence = idom;
            break;
        } else if pattern.as_bytes()[ipat] != domain.as_bytes()[ipat] && 
                pattern.as_bytes()[ipat] != '?' as u8 {
            return false;                  // "abc" doesn't match "abd".
        }
        ipat += 1;                        // Everything's a match, so far.
    }
    // Find any further wildcards and any further matching sequences.
    loop {
        if pattern.len() > ipat && pattern.as_bytes()[ipat] == '*' as u8 {
            // Got wild again.
            loop {
                ipat += 1;
                if pattern.len() <= ipat {
                    return true;           // "ab*c*" matches "abcd".
                } 
                if pattern.as_bytes()[ipat] != '*' as u8 {
                    break;
                }
            }
            if domain.len() <= idom {
                return false;              // "*bcd*" doesn't match "abc".
            }
            // Search for the next prospective match.
            if pattern.as_bytes()[ipat] != '?' as u8 {
                while domain.len() > idom && 
                      pattern.as_bytes()[ipat] != domain.as_bytes()[idom] {
                    idom += 1;
                    if domain.len() <= idom{
                        return false;      // "a*b*c" doesn't match "ab".
                    }
                }
            }
            // Keep the new fallback positions.
            ipat_sequence = ipat;
            idom_sequence = idom;
        } else {
            // The equivalent portion of the upper loop is really simple.
            if domain.len() <= idom {
                if pattern.len() <= ipat {
                    return true;           // "*b*c" matches "abc".
                }
                return false;              // "*bcd" doesn't match "abc".
            }
            if pattern.len() <= ipat ||
               pattern.as_bytes()[ipat] != domain.as_bytes()[idom] && 
               pattern.as_bytes()[ipat] != '?' as u8 {
                // A fine time for questions.
                while pattern.len() > ipat_sequence && 
                      pattern.as_bytes()[ipat_sequence] == '?' as u8 {
                    ipat_sequence += 1;
                    idom_sequence += 1;
                }
                ipat = ipat_sequence;
                // Fall back, but never so far again.
                loop {
                    idom_sequence += 1;
                    if domain.len() <= idom_sequence {
                        if pattern.len() <= ipat {
                            return true;   // "*a*b" matches "ab".
                        } else {
                            return false;  // "*a*b" doesn't match "ac".
                        }
                    }
                    if pattern.len() > ipat && pattern.as_bytes()[ipat] == 
                       domain.as_bytes()[idom_sequence] {
                        break;
                    }
                }
                idom = idom_sequence;
            }
        }
        // Another check for the end, at the end.
        if domain.len() <= idom {
            if pattern.len() <= ipat {
                return true;           // "*bc" matches "abc".
            }
            return false;              // "*bc" doesn't match "abcd".
        }
        ipat += 1;                    // Everything's still a match.
        idom += 1;
    }
}

pub fn check_domain(domain: &str, rules: &[Rule]) -> Action {
    let mut final_priority: u32 = 0;
    let mut final_action = Action::Allow;
    let mut found_match = false;

    for rule in rules {
        if matches_url(&rule.pattern, domain)
            && (!found_match || rule.priority > final_priority)
        {
            final_priority = rule.priority;
            final_action = rule.action;
            found_match = true;
        }
    }
    final_action
}

pub fn parse_rules_json(json_file: &str) -> Result<Vec<Rule>, Box<dyn Error>> {
    let file = File::open(json_file)?;
    let reader = BufReader::new(file);
    let rules: Vec<Rule> = serde_json::from_reader(reader)?;
    Ok(rules)
}

pub fn check_for_match(input: &str) -> Result<Action, Box<dyn Error>> {
    
    let rules = parse_rules_json("rules.json")?;
    Ok(check_domain(input, &rules))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test cases for matches_url function

    #[test]
    fn test_matches_example() {
        assert_eq!(matches_url("example.com", "example.com"), true);
    }
    #[test]
    fn test_matches_www() {
        assert_eq!(matches_url("example.com", "www.example.com"), false);
    }
    #[test]
    fn test_matches_wildcard() {
        assert_eq!(matches_url("*", "example.com"), true);
    }
    #[test]
fn test_check_domain() {
    let rules = vec![
        Rule {
            pattern: "*test.com".to_string(),
            action: Action::Block,
            priority: 1,
        },
    ];
    assert_eq!(check_domain("www.test.com", &rules), Action::Block);
}
#[test]
fn test_check_domain_no_match() {
    let rules = vec![
        Rule {
            pattern: "example.com".to_string(),
            action: Action::Block,
            priority: 100,
        },
    ];

    assert_eq!(check_domain("google.com", &rules), Action::Allow);
}

#[test]
fn test_check_domain_single_match() {
    let rules = vec![
        Rule {
            pattern: "example.com".to_string(),
            action: Action::Block,
            priority: 100,
        },
    ];

    assert_eq!(check_domain("example.com", &rules), Action::Block);
}

#[test]
fn test_check_domain_higher_priority_wins() {
    let rules = vec![
        Rule {
            pattern: "*example.com".to_string(),
            action: Action::Block,
            priority: 100,
        },
        Rule {
            pattern: "google.example.com".to_string(),
            action: Action::Allow,
            priority: 200,
        },
    ];

    assert_eq!(
        check_domain("google.example.com", &rules),
        Action::Allow
    );
}

#[test]
fn test_check_domain_lower_priority_does_not_override() {
    let rules = vec![
        Rule {
            pattern: "google.example.com".to_string(),
            action: Action::Allow,
            priority: 200,
        },
        Rule {
            pattern: "*example.com".to_string(),
            action: Action::Block,
            priority: 100,
        },
    ];

    assert_eq!(
        check_domain("google.example.com", &rules),
        Action::Allow
    );
}

#[test]
fn test_check_domain_equal_priority_first_rule_wins() {
    let rules = vec![
        Rule {
            pattern: "*example.com".to_string(),
            action: Action::Block,
            priority: 100,
        },
        Rule {
            pattern: "google.example.com".to_string(),
            action: Action::Allow,
            priority: 100,
        },
    ];

    assert_eq!(
        check_domain("google.example.com", &rules),
        Action::Block
    );
}
#[test]
fn test_check_domain_priority_zero() {
    let rules = vec![
        Rule {
            pattern: "example.com".to_string(),
            action: Action::Block,
            priority: 0,
        },
    ];

    assert_eq!(
        check_domain("example.com", &rules),
        Action::Block
    );
}

#[test]
fn test_check_domain_same_rule_same_priority() {
    let rules = vec![
        Rule {
            pattern: "example.com".to_string(),
            action: Action::Block,
            priority: 100,
        },
        Rule {
            pattern: "example.com".to_string(),
            action: Action::Allow,
            priority: 100,
        },
    ];

    assert_eq!(
        check_domain("example.com", &rules),
        Action::Block
    );
}

// Test cases for parse_rules_json function

#[test]
fn test_parse_rules_json() {
    let rules = parse_rules_json("rules.json").unwrap();

    assert!(!rules.is_empty());

    let example_rule = rules
        .iter()
        .find(|rule| rule.pattern == "example.com")
        .unwrap();

    assert_eq!(example_rule.action, Action::Block);
    assert_eq!(example_rule.priority, 100);
}

#[test]
fn test_parse_rules_json_missing_file() {
    let result = parse_rules_json("this_file_does_not_exist.json");

    assert!(result.is_err());
}

#[test]
fn test_check_for_match_blocked_domain() {
    let result = check_for_match("example.com");

    assert_eq!(result.unwrap(), Action::Block);
}

#[test]
fn test_check_for_match_allowed_domain() {
    let result = check_for_match("google.com");

    assert_eq!(result.unwrap(), Action::Allow);
}

#[test]
fn test_check_for_match_priority() {
    let result = check_for_match("google.example.com");

    assert_eq!(result.unwrap(), Action::Allow);
}
}