use crate::prelude::*;
use std::io::stdin;

pub fn ask_for_confirmation(prompt: &str) -> AppResult<bool> {
    println!("{}", prompt);
    println!("confirm by enter (y or yes). Or type something else to cancel.");
    let mut may_confirmation = String::new();
    let to_read_from = stdin();
    to_read_from.read_line(&mut may_confirmation)?;

    let trimmed = may_confirmation.trim();
    if trimmed == "y" || trimmed == "yes" {
        Ok(true)
    } else {
        Ok(false)
    }
}
