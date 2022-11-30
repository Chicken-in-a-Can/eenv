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
