// Get functions from local files
#[path = "tools.rs"] mod tools;
#[path = "get_info.rs"] mod get_info;

// Imports
use std::path::Path;
use std::io::{BufReader, prelude::*};
use std::fs::File;

// Get paths (`$PATH` in bash)
#[allow(dead_code)]
pub fn get_path() -> Vec<String>{
    // Get username
    let uname: String = get_info::get_username();
    // Create `.eenv_profile` if it doesn't exists
    if !Path::new(format!("/home/{}/.eenv_profile", uname).as_str()).exists(){
        let _result = File::create(format!("/home/{}/.eenv_profile", uname).as_str());
    }
    // Read in profile file to Vec
    let eenv_profile_read = File::open(format!("/home/{}/.eenv_profile", uname).as_str()).expect("file `.eenv_profile` was unable to be opened");
    let buffer = BufReader::new(eenv_profile_read);
    let eenv_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    // Run through profile lines
    for line in eenv_vec{
        // Get first word of line
        let seperated_line = tools::seperate_first_substr(line);
        // if seperated line == "path", return the after it.
        if seperated_line.0 == "path"{
            let split = seperated_line.1.split(" ");
            return split.map(|w| w.to_owned()).collect();
        }
    }
    // If no path specified, path = `/bin` `/usr/bin`
    return vec!["/bin".to_owned(), "/usr/bin".to_owned()]
}

// Swap out home key (typically `~`) with home directory
#[allow(dead_code)]
pub fn home_swap(pre_swap: String) -> String{
    // Get username
    let uname = get_info::get_username();
    // Initialize return string
    let mut return_string = "".to_owned();
    // Get home key to swap, usually `~`
    let swap_key = get_home_key();
    let mut i: usize = 0;
    // Iterate through command, look for char in correct situations
    while i < pre_swap.len(){
        if i >= 1{
            // If sequence ` ~` detected, swap out
            if pre_swap.chars().nth(i - 1).unwrap() == ' ' && pre_swap.chars().nth(i).unwrap() == swap_key{
                return_string = format!("{}{}", return_string, format!("/home/{}", uname));
            }
            else{
                // Else, just add to return string
                return_string = format!("{}{}", return_string, pre_swap.chars().nth(i).unwrap());
            }
        }
        else{
            // If sequence '~' detected and at start of line, swap out
            if pre_swap.chars().nth(i).unwrap() == swap_key{
                return_string = format!("{}{}", return_string, format!("/home/{}", uname));
            }
            else{
                // Else, just add to retrun string
                return_string = format!("{}{}", return_string, pre_swap.chars().nth(i).unwrap());
            }
        }
        // Increment i
        i += 1;
    }
    // Return String at end
    return return_string;
}

// Swap out here key (Usually '.')
#[allow(dead_code)]
pub fn here_swap(pre_swap: String) -> String{
    // Get current directory, full not basename
    let current = get_info::get_current_dir(false);
    // Create blank return string
    let mut return_string = "".to_owned();
    // Get here key. Usually `.`
    let swap_key = get_here_key();
    let mut i: usize = 0;
    // Loop through command
    while i < pre_swap.len(){
        if i >= 1{
            // If sequence ` .` detected, swap out
            if pre_swap.chars().nth(i - 1).unwrap() == ' ' && pre_swap.chars().nth(i).unwrap() == swap_key{
                return_string = format!("{}{}", return_string, format!("{}", current));
            }
            // Else, just add to return String
            else{
                return_string = format!("{}{}", return_string, pre_swap.chars().nth(i).unwrap());
            }
        }
        else{
            // If sequence `.` detected at start, swap out
            if pre_swap.chars().nth(i).unwrap() == swap_key{
                return_string = format!("{}{}", return_string, format!("{}", current));
            }
            // Else, just add to return String
            else{
                return_string = format!("{}{}", return_string, pre_swap.chars().nth(i).unwrap());
            }
        }
        // Increment i
        i += 1;
    }
    // Return return String
    return return_string;
}

// Swap out back key (Usually `..`)
#[allow(dead_code)]
pub fn back_swap(pre_swap: String) -> String{
    // Get full path and replace `/{basename"` with ``
    let current = get_info::get_current_dir(false).replace(format!("/{}", get_info::get_current_dir(true)).as_str(), "");
    // Initialize return string
    let mut return_string = "".to_owned();
    // Get back key. Usually '..'
    let swap_key = get_back_key();
    let mut i: usize = 0;
    // Iterate through command
    while i < pre_swap.len(){
        if i >= 2{
            // If sequence ` ..` found, swap out
            if i < pre_swap.len() - 1 && pre_swap.chars().nth(i - 1).unwrap() == ' ' && pre_swap[(i)..(i+2)] == swap_key{
                return_string = format!("{}{}", return_string, format!("{}", current));
                i += 1;
            }
            else{
                // Else, just add to return string
                return_string = format!("{}{}", return_string, pre_swap.chars().nth(i).unwrap());
            }
        }
        else if i >= 1{
            // If sequence `..` found at start of line, swap out
            if i < pre_swap.len() - 1 && pre_swap[(i)..(i+2)] == swap_key{
                return_string = format!("{}{}", return_string, format!("{}", current));
                i += 1;
            }
            else{
                // Else, just add to return string
                return_string = format!("{}{}", return_string, pre_swap.chars().nth(i).unwrap());
            }
        }
        else{
            // Else, just add to return string
            return_string = format!("{}{}", return_string, pre_swap.chars().nth(i).unwrap());
        }
        // Inccrement i
        i += 1;
    }
    // Return return String
    return return_string;
}

// Get home key (Usually `~`)
fn get_home_key() -> char{
    // Get username
    let uname: String = get_info::get_username();
    // If `~/.eenv_profile` doesn't exist, create it
    if !Path::new(format!("/home/{}/.eenv_profile", uname).as_str()).exists(){
        let _result = File::create(format!("/home/{}/.eenv_profile", uname).as_str());
    }
    // Read in `~/.eenv_profile` to Vec
    let eenv_profile_read = File::open(format!("/home/{}/.eenv_profile", uname).as_str()).expect("file `.eenv_profile` was unable to be opened");
    let buffer = BufReader::new(eenv_profile_read);
    let eenv_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    // Read through lines in profile Vec
    for line in eenv_vec{
        // Seperate line
        let seperated_line = tools::seperate_first_substr(line);
        // If first work in line is home, set home key
        if seperated_line.0 == "home"{
            return seperated_line.1.chars().nth(0).unwrap();
        }
    }
    // Else, return `~` as home key
    return '~'
}

// Get here key (Usually `.`)
fn get_here_key() -> char{
    // Get username
    let uname: String = get_info::get_username();
    // If `~/.eenv_profile` doesn't exists, create it
    if !Path::new(format!("/home/{}/.eenv_profile", uname).as_str()).exists(){
        let _result = File::create(format!("/home/{}/.eenv_profile", uname).as_str());
    }
    // Read in `~/.eenv_profile` to Vec
    let eenv_profile_read = File::open(format!("/home/{}/.eenv_profile", uname).as_str()).expect("file `.eenv_profile` was unable to be opened");
    let buffer = BufReader::new(eenv_profile_read);
    let eenv_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    // Read through lines in profile Vec
    for line in eenv_vec{
        // Seperate line
        let seperated_line = tools::seperate_first_substr(line);
        // If fisrt word in line is here, set here key
        if seperated_line.0 == "here"{
            return seperated_line.1.chars().nth(0).unwrap();
        }
    }
    // Else, return '.' as here key
    return '.'
}

// Get back key (Usually `..`)
fn get_back_key() -> String{
    // Get username
    let uname: String = get_info::get_username();
    // If `~/.eenv_profile` doesn't exist, create it
    if !Path::new(format!("/home/{}/.eenv_profile", uname).as_str()).exists(){
        let _result = File::create(format!("/home/{}/.eenv_profile", uname).as_str());
    }
    // Read in `~/.eenv_profile` to Vec
    let eenv_profile_read = File::open(format!("/home/{}/.eenv_profile", uname).as_str()).expect("file `.eenv_profile` was unable to be opened");
    let buffer = BufReader::new(eenv_profile_read);
    let eenv_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    // Read through lines in profile Vec
    for line in eenv_vec{
        // Seperate line
        let seperated_line = tools::seperate_first_substr(line);
        // If first work in line is back, set back key
        if seperated_line.0 == "back" && seperated_line.1.len() >= 2{
            return format!("{}{}", seperated_line.1.chars().nth(0).unwrap(), seperated_line.1.chars().nth(1).unwrap());
        }
    }
    // Else, return `..` as back key
    return "..".to_owned()
}
