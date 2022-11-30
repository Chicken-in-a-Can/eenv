mod get_info;
mod alias;

use rustyline::Editor;
use colored::Colorize;
use std::process::{Command, exit};
use std::env::set_current_dir;
use std::path::Path;
use std::collections::HashMap;

fn main(){
    let aliases: HashMap<String, String> = alias::get_aliases();
    let paths = vec!["/usr/bin", "/bin"];
    let hostname = get_info::get_hostname();
    let uname = get_info::get_username();
    let mut reader = Editor::<()>::new().unwrap();
    loop{
        let current_user_dir_basename: String = get_info::get_current_dir(true);
        let prompt = format!("{}{}{}{} {}{}{}", format!("[").green(), uname.green(), format!("@").green(), hostname.green(), current_user_dir_basename.white(), format!("]$").green(), format!(" ").white());
        let readline = alias::alias_swap(reader.readline(prompt.as_str()).unwrap(), aliases.clone());
        let mut parts = readline.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        match command{
            "exit" => {
                exit(0);
            }
            "cd" => {
                let path_to_nav = args.clone().next().unwrap();
                set_current_dir(path_to_nav);
            }
            "hostname" => {
                println!("{}", hostname);
            }
            _ => {
                if check_command_existence(command.clone(), paths.clone()){
                    let mut child = Command::new(command)
                        .args(args)
                        .spawn()
                        .unwrap();
                    child.wait();
                }
                else{
                    command_not_exist(command);
                }
            }
        }
    }
}
fn check_command_existence(command: &str, paths: Vec<&str>) -> bool{
    let mut return_bool = false;
    for single_path in paths{
        if Path::new(format!("{}/{}", single_path.clone(), command.clone()).as_str()).exists(){
            return_bool = true;
        }
    }
    return return_bool;
}
fn command_not_exist(command: &str){
    println!("command {} could not be run", command);
}
