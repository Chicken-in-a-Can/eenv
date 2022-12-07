// Get functions from local files
#[path = "tools.rs"] mod tools;
#[path = "get_info.rs"] mod get_info;

// Imports
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use colored::Colorize;

// Return prompt tuple. First String is actual prompt, Second is just the prompt as in file
#[allow(dead_code)]
pub fn get_prompt() -> (String, String){
    // Get username
    let uname = get_info::get_username();
    // Create `~/.eenv_profile` if doesn't exist
    if !Path::new(format!("/home/{}/.eenv_profile", uname).as_str()).exists(){
        let _result = File::create(format!("/home/{}/.eenv_profile", uname).as_str());
    }
    // Read in `~/.eenv_profile` to Vec
    let eenv_profile_read = File::open(format!("/home/{}/.eenv_profile", uname).as_str()).expect("file `.eenv_profile` was unable to to be opened");
    let buffer = BufReader::new(eenv_profile_read);
    let eenv_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    // Read through lines
    for line in eenv_vec{
        // Seperate line
        let seperated_line = tools::seperate_first_substr(line);
        // If first word of line is prompt, get prompt;
        if seperated_line.0 == "prompt"{
            return (prompt_gen(seperated_line.1.clone()), seperated_line.1);
        }
    }
    // If no prompt specified, set prompt to `> `
    return ("> ".to_owned(), "> ".to_owned());
}

// Generate prompt
#[allow(dead_code)]
pub fn prompt_gen(prompt: String) -> String{
    // Initialize variables
    let mut final_string = "".to_owned();
    let mut uncolored_substring = "".to_owned();
    let mut in_special = false;
    let mut special_type = ' ';
    let special_chars = vec!['|', '{'];
    // Loop through prompt string
    let mut i: usize = 0;
    while i < prompt.len(){
        // If not in {} or ||
        if !in_special{
            // If next char isn't a special one
            if !special_chars.contains(&prompt.chars().nth(i).unwrap()){
                uncolored_substring = format!("{}{}", uncolored_substring, prompt.chars().nth(i).unwrap());
            }
            // If next char is a special one
            else{
                in_special = true;
                // Check which special char it is
                match prompt.chars().nth(i).unwrap(){
                    '|' => special_type = 'c',
                    '{' => special_type = 'i',
                    _ => special_type = ' '
                }
            }
        }
        // If in {} or ||
        else{
            // Check which special char it is
            match special_type{
                'i' => {
                    uncolored_substring = format!("{}{}", uncolored_substring, prompt_info_handler(prompt.chars().nth(i).unwrap()));
                },
                'c' => {
                    final_string = format!("{}{}", final_string, color_substring(uncolored_substring, prompt.chars().nth(i).unwrap()));
                    uncolored_substring = "".to_owned();
                },
                _ => (),
            }
            // Skip past closing special char
            i += 1;
            // Reset variables
            special_type = ' ';
            in_special = false;
        }
        // Increment loop
        i += 1;
    }
    // If no color at end, just do it default
    if uncolored_substring != ""{
        final_string = format!("{}{}", final_string, uncolored_substring);
    }
    // Return final String
    return final_string;
}

// Color handler
#[allow(unused_assignments)]
fn color_substring(substr: String, color: char) -> String{
    // Initialize String
    let mut colored_substr = String::new();
    match color{
        // If 'w', set color to white
        'w' => colored_substr = format!("{}", substr.white()),
        // If 'g', set color to green
        'g' => colored_substr = format!("{}", substr.green()),
        // If 'r', set color to red
        'r' => colored_substr = format!("{}", substr.red()),
        // If 'y', set color to yellow
        'y' => colored_substr = format!("{}", substr.yellow()),
        // If 'b', set color to blue
        'b' => colored_substr = format!("{}", substr.blue()),
        // If 'm', set color to magenta
        'm' => colored_substr = format!("{}", substr.magenta()),
        // if 'p', set color to purple
        'p' => colored_substr = format!("{}", substr.purple()),
        // If 'c', set color to cyan
        'c' => colored_substr = format!("{}", substr.cyan()),
        // Else, don't change color
        _ => colored_substr = format!("{}", substr),
    }
    // Return colored String
    return colored_substr;
}

// Special info handler
#[allow(unused_assignments)]
fn prompt_info_handler(info_char: char) -> String{
    // Initialize return String
    let mut return_string = "".to_owned();
    // Match given char
    match info_char{
        // If 'u', return username
        'u' => return_string = get_info::get_username(),
        // If 'h', return hostname
        'h' => return_string = get_info::get_hostname(),
        // If 'b', return current directory basename
        'b' => return_string = get_info::get_current_dir(true),
        // If 'd', return current directory
        'd' => return_string = get_info::get_current_dir(false),
        // If other, return char
        _ => return_string = format!("{}", info_char),
    }
    // Return return String
    return return_string;
}
