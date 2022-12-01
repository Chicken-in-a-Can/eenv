mod get_info;
mod alias;
mod prompt;
mod commandler;

use rustyline::Editor;
use std::collections::HashMap;

fn main(){
    let prompt_and_read: (String, String) = prompt::get_prompt();
    let read_prompt = prompt_and_read.clone().1;
    let aliases: HashMap<String, String> = alias::get_aliases();
    let mut reader = Editor::<()>::new().unwrap();
    loop{
        let prompt = prompt::prompt_gen(read_prompt.clone());
        let readline = alias::alias_swap(reader.readline(prompt.as_str()).unwrap(), aliases.clone());
        commandler::commandler(readline);
    }
}
