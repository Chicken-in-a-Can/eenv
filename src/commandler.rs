#[path = "alias.rs"] mod alias;
#[path = "get_info.rs"] mod get_info;
#[path = "prompt.rs"] mod prompt;

use std::env::set_current_dir;
use std::process::{Command, exit};
use std::path::Path;

#[allow(dead_code)]
pub fn commandler(line_read: String){
    let paths = vec!["/usr/bin".to_owned(), "/bin".to_owned()];
    let mut parts = line_read.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;
    match command{
        "exit" => {
            exit(0);
        }
        "cd" => {
            let path_to_nav = args.clone().next().unwrap();
            let _result = set_current_dir(path_to_nav).unwrap();
        }
        "hostname" => {
            println!("{}", get_info::get_hostname());
        }
        "prompt" => {
            println!("{}", prompt::get_prompt().0);
        }
        _ => {
            let command_tuple = check_command_existence(command.clone(), paths.clone());
            if command_tuple.0{
                let full_path_command = command_tuple.1;
                let mut child = Command::new(full_path_command)
                    .args(args)
                    .spawn()
                    .unwrap();
                let _result = child.wait().unwrap();
            }
            else{
                command_not_exist(command);
            }
        }
    }
}
fn check_command_existence(command: &str, paths: Vec<String>) -> (bool, String){
    let mut return_tuple = (false, "".to_owned());
    for single_path in paths{
        if Path::new(format!("{}/{}", single_path.clone(), command.clone()).as_str()).exists(){
            return_tuple = (true, format!("{}/{}", single_path, command));
        }
    }
    return return_tuple
}
fn command_not_exist(command: &str){
    println!("command {} could not be run", command);
}
