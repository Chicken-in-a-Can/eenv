use rustyline::Editor;
use colored::Colorize;
use rustyline::error::ReadlineError;
use std::process::{self, Command, exit};
use std::collections::HashMap;
use std::io::{self, stdout, stdin, Write, Read};
use std::fs::{self, File};
use std::str::{self, from_utf8};
use std::env::{self, current_dir, set_current_dir};
use std::path::{self, Path};

fn main(){
    let mut aliases: HashMap<&str, &str> = HashMap::new();
    aliases.insert("ls", "ls --color=auto");
    aliases.insert("grep", "grep --color==auto");
    let paths = vec!["/usr/bin", "/bin"];
    let mut os_seperator = "/";
    let mut etc_hostname = File::open("/etc/hostname").expect("/etc/hostname could not be opened");
    let mut hostname = String::new();
    etc_hostname.read_to_string(&mut hostname).expect("Unable to read /etc/hostname");
    let mut uname_command = Command::new("whoami");
    let mut uname: String = from_utf8(&uname_command.output().unwrap().stdout[..]).unwrap().to_string();
    hostname.pop();
    uname.pop();
    let mut reader = Editor::<()>::new().unwrap();
    loop{
        let mut current_user_dir: String = current_dir().unwrap().to_str().expect("Current directory could not be read").to_string();
        let current_user_dir_basename_arr: Vec<&str> = current_user_dir.split(os_seperator.clone()).collect();
        let current_user_dir_basename: &str = current_user_dir_basename_arr[current_user_dir_basename_arr.len() - 1];
        let prompt = format!("{}{}{}{} {}{}{}", format!("[").green(), uname.green(), format!("@").green(), hostname.green(), current_user_dir_basename.white(), format!("]$").green(), format!(" ").white());
        let readline = reader.readline(prompt.as_str()).unwrap();
        stdout().flush();
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
