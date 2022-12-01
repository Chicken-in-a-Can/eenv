#[path = "tools.rs"] mod tools;
#[path = "get_info.rs"] mod get_info;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
pub fn get_aliases() -> HashMap<String, String>{
    let mut aliases: HashMap<String, String> = HashMap::new();
    let uname = get_info::get_username();
    if !Path::new(format!("/home/{}/.eenv_aliases", uname).as_str()).exists(){
        let _result = File::create(format!("/home/{}/.eenv_aliases", uname));
    }
    let alias_read = File::open(format!("/home/{}/.eenv_aliases", uname).as_str()).expect("Alias file `~/.eenv_aliases` was unable to to be opened");
    let buffer = BufReader::new(alias_read);
    let alias_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    for alias in alias_vec{
        let mut firststr: String = "".to_owned();
        let mut secondstr: String = "".to_owned();
        let mut spaced: bool = false;
        if alias.chars().nth(0).unwrap() != '#' && alias != ""{
            for c in alias.chars(){
                if !spaced && c != ' '{
                    firststr = format!("{}{}", firststr, c);
                }
                else if !spaced && c == ' '{
                    spaced = true;
                }
                else{
                    secondstr = format!("{}{}", secondstr, c);
                }
            }
            aliases.insert(firststr.clone(), secondstr.clone());
        }
    }
    return aliases;
}
#[allow(dead_code)]
pub fn alias_swap(long_command: String, aliases: HashMap<String, String>) -> String{
    let mut split_command = tools::seperate_first_substr(long_command.clone());
    if aliases.contains_key(&split_command.0.clone()){
        split_command.0 = aliases.get(&split_command.0.clone()).unwrap().clone();
    }
    let return_final: String = format!("{} {}", split_command.0, split_command.1);
    return return_final;
}
