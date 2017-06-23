/*
Rust - Std Misc Path
Licence    : GNU GPL v3 or later
Author     : Aurélien DESBRIÈRES
Mail       : aurelien(at)hackers(dot)camp
Created on : June 2017

Write with Emacs-nox ───────────────┐
Rust ───────────────────────────────┘
*/

use std::path::Path;
 
fn main() {
    // Create a `Path` from a `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Show`able structure
    //let display = path.display();
    println!("{}", path.display());

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns the new path
    let new_path = path.join("a").join("b");

    // Convert the path into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

// Other methos:
// posix::Path
// windows::Path
