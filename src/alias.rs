// Get functions from local files
#[path = "tools.rs"] mod tools;
#[path = "get_info.rs"] mod get_info;

// Imports
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

// Gets aliases from `~/.eenv_aliases` file
#[allow(dead_code)]
pub fn get_aliases() -> HashMap<String, String>{
    // Initialize empty String + String hashmap
    let mut aliases: HashMap<String, String> = HashMap::new();
    // Get username
    let uname = get_info::get_username();
    // Create `~/.eenv_aliases` file if it doesn't exist
    if !Path::new(format!("/home/{}/.eenv_aliases", uname).as_str()).exists(){
        let _result = File::create(format!("/home/{}/.eenv_aliases", uname));
    }
    // Read `~/.eenv_aliases` in to vector
    let alias_read = File::open(format!("/home/{}/.eenv_aliases", uname).as_str()).expect("Alias file `~/.eenv_aliases` was unable to to be opened");
    let buffer = BufReader::new(alias_read);
    let alias_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    // Get individual aliases
    for alias in alias_vec{
        // Check if not commented out
        if alias.chars().nth(0).unwrap() != '#' && alias != ""{
            // Split and push into hashmap
            let string_tuple = tools::seperate_first_substr(alias.clone());
            aliases.insert(string_tuple.0, string_tuple.1);
        }
    }
    // Return alias hashmap
    return aliases;
}
// Swap out aliases for given command
#[allow(dead_code)]
pub fn alias_swap(long_command: String, aliases: HashMap<String, String>) -> String{
    // Seperate command into multiple parts
    let mut split_command = tools::seperate_first_substr(long_command.clone());
    // If first part of alias command is in hashmap, swap it out
    if aliases.contains_key(&split_command.0.clone()){
        split_command.0 = aliases.get(&split_command.0.clone()).unwrap().clone();
    }
    // Recombine tuple
    let return_final: String = format!("{} {}", split_command.0, split_command.1);
    // Return swapped string
    return return_final;
}
