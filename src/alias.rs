use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
pub fn get_aliases() -> HashMap<String, String>{
    let mut aliases: HashMap<String, String> = HashMap::new();
    let alias_read = File::open("./.aliases").expect("Alias file `.aliases` was unable to to be opened");
    let buffer = BufReader::new(alias_read);
    let alias_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    for alias in alias_vec{
        let mut firststr: String = "".to_owned();
        let mut secondstr: String = "".to_owned();
        let mut spaced: bool = false;
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
    return aliases;
}
#[allow(dead_code)]
pub fn alias_swap(long_command: String, aliases: HashMap<String, String>) -> String{
    let mut space_found: bool = false;
    let mut starting_substring = "".to_owned();
    let mut ending_substring = "".to_owned();
    for c in long_command.chars(){
        if !space_found{
            if c != ' '{
                starting_substring = format!("{}{}", starting_substring, c);
            }
            else{
                space_found = true;
            }
        }
        else{
            ending_substring = format!("{}{}", ending_substring, c);
        }
    }
    if aliases.contains_key(&starting_substring.clone()){
        starting_substring = aliases.get(&starting_substring.clone()).unwrap().clone();
    }
    let return_final: String = format!("{} {}", starting_substring, ending_substring);
    return return_final;
}
