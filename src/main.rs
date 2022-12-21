extern crate colored;
extern crate emoji_commit_type;

use emoji_commit_type::CommitType;

use colored::*;

use std::{env, process::exit};

use std::error::Error;
use std::fmt;

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
    for commit_type in CommitType::iter_variants() {
        if commit.starts_with(commit_type.emoji()) {
            return Ok(());
        }
    }

    let one_of = CommitType::iter_variants()
        .map(|commit_type| commit_type.emoji())
        .collect::<Vec<&str>>()
        .join(", ");
    Err(EmojiCommitError::new(&format!(
        "Commit does not start with a valid commit type: {}",
        one_of
    )))
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
        // Err(e EmojiCommitError) => {
        //     eprintln!("Error: {}", e);
        //     exit(1);
        // }
        Err(e) => {
            eprintln!("{} {}", "Error:".red(), e.to_string().red());
            exit(1);
        }
    }
}
