#[path = "tools.rs"] mod tools;
#[path = "get_info.rs"] mod get_info;

use std::path::Path;
use std::io::{BufReader, prelude::*};
use std::fs::File;

#[allow(dead_code)]
pub fn get_path() -> Vec<String>{
    let uname: String = get_info::get_username();
    if !Path::new(format!("/home/{}/.eenv_profile", uname).as_str()).exists(){
        let _result = File::create(format!("/home/{}/.eenv_profile", uname).as_str());
    }
    let eenv_profile_read = File::open(format!("/home/{}/.eenv_profile", uname).as_str()).expect("file `.eenv_profile` was unable to be opened");
    let buffer = BufReader::new(eenv_profile_read);
    let eenv_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    for line in eenv_vec{
        let seperated_line = tools::seperate_first_substr(line);
        if seperated_line.0 == "path"{
            let split = seperated_line.1.split(" ");
            return split.map(|w| w.to_owned()).collect();
        }
    }
    return vec!["/bin".to_owned(), "/usr/bin".to_owned()]
}


#[allow(dead_code)]
pub fn home_swap(pre_swap: String) -> String{
    let uname = get_info::get_username();
    let mut return_string = "".to_owned();
    let swap_key = get_home_key();
    let mut i: usize = 0;
    while i < pre_swap.len(){
        if i >= 1{
            if pre_swap.chars().nth(i - 1).unwrap() == ' ' && pre_swap.chars().nth(i).unwrap() == swap_key{
                return_string = format!("{}{}", return_string, format!("/home/{}", uname));
            }
            else{
                return_string = format!("{}{}", return_string, pre_swap.chars().nth(i).unwrap());
            }
        }
        else{
            if pre_swap.chars().nth(i).unwrap() == swap_key{
                return_string = format!("{}{}", return_string, format!("/home/{}", uname));
            }
            else{
                return_string = format!("{}{}", return_string, pre_swap.chars().nth(i).unwrap());
            }
        }
        i += 1;
    }
    return return_string;
}

fn get_home_key() -> char{
    let uname: String = get_info::get_username();
    if !Path::new(format!("/home/{}/.eenv_profile", uname).as_str()).exists(){
        let _result = File::create(format!("/home/{}/.eenv_profile", uname).as_str());
    }
    let eenv_profile_read = File::open(format!("/home/{}/.eenv_profile", uname).as_str()).expect("file `.eenv_profile` was unable to be opened");
    let buffer = BufReader::new(eenv_profile_read);
    let eenv_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    for line in eenv_vec{
        let seperated_line = tools::seperate_first_substr(line);
        if seperated_line.0 == "home"{
            return seperated_line.1.chars().nth(0).unwrap();
        }
    }
    return '~'
}
