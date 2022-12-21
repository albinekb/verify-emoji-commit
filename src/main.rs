extern crate colored;
extern crate emoji_commit_type;

use colored::*;

use std::{env, process::exit};

use std::error::Error;
use std::fmt;

mod commit_rules;

#[derive(Debug)]
struct EmojiCommitError {
    details: String,
}

impl EmojiCommitError {
    fn new(msg: &str) -> EmojiCommitError {
        EmojiCommitError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for EmojiCommitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for EmojiCommitError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn verify_commit(commit: &str) -> Result<(), EmojiCommitError> {
    let checked = commit_rules::check_message_with_emoji(&commit);

    let not_passed_rules = checked.filter(|rule| !rule.pass);

    let result = not_passed_rules
        .map(|rule| rule.description)
        .collect::<Vec<_>>()
        .join("\r\n");

    if result.len() == 0 {
        return Ok(());
    }

    Err(EmojiCommitError::new(&result))
}

fn main() {
    let print_success = env::var("PRINT_SUCCESS")
        .unwrap_or("true".to_string())
        .contains("true")
        != false;

    let all_args: Vec<String> = env::args().collect();

    let mut args = all_args.clone();
    args.remove(0);
    let commit: &String = &args.join(" ").clone();
    let result = verify_commit(commit);

    match result {
        Ok(_) => {
            if print_success {
                println!("{}", "Commit is valid".green());
            }
            exit(0);
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red(), e.to_string().red());
            exit(1);
        }
    }
}
