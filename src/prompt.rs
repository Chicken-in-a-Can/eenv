#[path = "tools.rs"] mod tools;
#[path = "get_info.rs"] mod get_info;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use colored::Colorize;

#[allow(dead_code)]
pub fn get_prompt() -> (String, String){
    let eenvrc_read = File::open("./.eenvrc").expect("Alias file `.eenvrc` was unable to to be opened");
    let buffer = BufReader::new(eenvrc_read);
    let bashrc_vec: Vec<String> = buffer.lines().map(|l| l.expect("Could not parse line")).collect();
    for line in bashrc_vec{
        let seperated_line = tools::seperate_first_substr(line);
        if seperated_line.0 == "prompt"{
            return (prompt_gen(seperated_line.1.clone()), seperated_line.1);
        }
    }
    return ("> ".to_owned(), "> ".to_owned());

}

#[allow(dead_code)]
pub fn prompt_gen(prompt: String) -> String{
    let mut final_string = "".to_owned();
    let mut uncolored_substring = "".to_owned();
    let mut in_special = false;
    let mut special_type = ' ';
    let special_chars = vec!['|', '{'];
    let mut i: usize = 0;
    while i < prompt.len(){
        if !in_special{
            if !special_chars.contains(&prompt.chars().nth(i).unwrap()){
                uncolored_substring = format!("{}{}", uncolored_substring, prompt.chars().nth(i).unwrap());
            }
            else{
                in_special = true;
                match prompt.chars().nth(i).unwrap(){
                    '|' => special_type = 'c',
                    '{' => special_type = 'i',
                    _ => special_type = ' '
                }
            }
        }
        else{
            match special_type{
                'i' => {
                    uncolored_substring = format!("{}{}", uncolored_substring, prompt_info_handler(prompt.chars().nth(i).unwrap()));
                },
                'c' => {
                    final_string = format!("{}{}", final_string, color_substring(uncolored_substring, prompt.chars().nth(i).unwrap()));
                    uncolored_substring = "".to_owned();
                },
                _ => (),
            }
            i += 1;
            special_type = ' ';
            in_special = false;
        }
        i += 1;
    }
    if uncolored_substring != ""{
        final_string = format!("{}{}", final_string, uncolored_substring);
    }
    return final_string;
}

fn color_substring(substr: String, color: char) -> String{
    let mut colored_substr = String::new();
    match color{
        'w' => colored_substr = format!("{}", substr.white()),
        'g' => colored_substr = format!("{}", substr.green()),
        'r' => colored_substr = format!("{}", substr.red()),
        'y' => colored_substr = format!("{}", substr.yellow()),
        'b' => colored_substr = format!("{}", substr.blue()),
        'm' => colored_substr = format!("{}", substr.magenta()),
        'p' => colored_substr = format!("{}", substr.purple()),
        'c' => colored_substr = format!("{}", substr.cyan()),
        _ => colored_substr = format!("{}", substr),
    }
    return colored_substr;
}

fn prompt_info_handler(info_char: char) -> String{
    let mut return_string = "".to_owned();
    match info_char{
        'u' => return_string = get_info::get_username(),
        'h' => return_string = get_info::get_hostname(),
        'b' => return_string = get_info::get_current_dir(true),
        'd' => return_string = get_info::get_current_dir(false),
        _ => return_string = format!("{}", info_char),
    }
    return return_string;
}
