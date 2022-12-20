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
        if commit.contains(commit_type.emoji()) {
            return Ok(());
        }
    }

    Err(EmojiCommitError::new(
        "Commit does not contain a valid commit type",
    ))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // let commit = &args[1];
    let mut vec = args.clone();

    vec.remove(0);

    let commit: &String = &vec.join(" ").clone();

    let result = verify_commit(commit);

    match result {
        Ok(_) => {
            println!("{} {}", "Commit:".green(), commit.green());
            exit(0);
        }
        // Err(e EmojiCommitError) => {
        //     eprintln!("Error: {}", e);
        //     exit(1);
        // }
        Err(e) => {
            println!("{} {}", "Commit:".red(), commit.red());
            eprintln!("{} {}", "Error:".red(), e.to_string().red());
            exit(1);
        }
    }
}
