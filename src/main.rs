mod get_info;
mod alias;
mod prompt;

use rustyline::Editor;
use std::process::{Command, exit};
use std::env::set_current_dir;
use std::path::Path;
use std::collections::HashMap;

fn main(){
    let prompt_and_read: (String, String) = prompt::get_prompt();
    let read_prompt = prompt_and_read.clone().1;
    let aliases: HashMap<String, String> = alias::get_aliases();
    let paths = vec!["/usr/bin", "/bin"];
    let mut reader = Editor::<()>::new().unwrap();
    loop{
        let prompt = prompt::prompt_gen(read_prompt.clone());
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
                let _result = set_current_dir(path_to_nav).unwrap();
            }
            "hostname" => {
                println!("{}", get_info::get_hostname());
            }
            "prompt" => {
                println!("{}", prompt::get_prompt().0);
            }
            _ => {
                if check_command_existence(command.clone(), paths.clone()){
                    let mut child = Command::new(command)
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
