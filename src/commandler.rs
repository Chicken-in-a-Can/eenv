// Get functions from local files

#[path = "alias.rs"] mod alias;
#[path = "get_info.rs"] mod get_info;
#[path = "prompt.rs"] mod prompt;
#[path = "get_path.rs"] mod get_path;

// Imports
use std::env::set_current_dir;
use std::process::{Command, exit};
use std::path::Path;

// Handle commands
#[allow(dead_code)]
pub fn commandler(line_read: String){
    // Swap out home key (~), back key (..), and here key (.)
    let line_swapped = get_path::here_swap(get_path::back_swap(get_path::home_swap(line_read)));
    // Get paths
    let paths = get_path::get_path();
    // Split command into parts
    let mut parts = line_swapped.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;
    // Match case for first part of command
    match command{
        // Exit when `exit` command typed
        "exit" => {
            exit(0);
        }
        // Change directory when `cd` command typed
        "cd" => {
            let path_to_nav = args.clone().next().unwrap();
            let _result = set_current_dir(path_to_nav).unwrap();
        }
        // Print hostname when `hostname` command typed
        "hostname" => {
            println!("{}", get_info::get_hostname());
        }
        // Show raw pompt when `prompt` command typed
        "prompt" => {
            println!("{}", prompt::get_prompt().0);
        }
        // Return paths array when `path` command typed
        "path" => {
            println!("{:?}", paths.clone());
        }
        // Else, try to run command on system
        _ => {
            // Check command existence
            let command_tuple = check_command_existence(command.clone(), paths.clone());
            // If command exists, run it
            if command_tuple.0{
                let full_path_command = command_tuple.1;
                let mut child = Command::new(full_path_command)
                    .args(args)
                    .spawn()
                    .unwrap();
                let _result = child.wait().unwrap();
            }
            // Else, pass to non-existence handler
            else{
                command_not_exist(command);
            }
        }
    }
}

// Check if command exists
fn check_command_existence(command: &str, paths: Vec<String>) -> (bool, String){
    // Initialize a bool + String tuple to return
    let mut return_tuple = (false, "".to_owned());
    // Set return tuple to absolute path if one is given; ./file works too
    if command.chars().nth(0).unwrap() == '/'{
        return_tuple = (true, command.to_owned());
    }
    else{
        // Run through path vector
        for single_path in paths{
            if Path::new(format!("{}/{}", single_path.clone(), command.clone()).as_str()).exists(){
                return_tuple = (true, format!("{}/{}", single_path, command));
            }
        }
    }
    // Return the return tuple
    return return_tuple
}

// Handle command non-existence
fn command_not_exist(command: &str){
    // Just print out that it couldn't be run
    println!("command {} could not be run", command);
}
