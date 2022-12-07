/* Executive Environment
 * by Ben Stidham
 *
 * A shell written in Rust for BossOS (https://github.com/Chicken-in-a-Can/the-executive-os)
 * Feel free to use and modify it. Just don't sue me if it breaks your system :)
 * Only tested on Arch (superior)
 */

// Get functions from local files
mod get_info;
mod alias;
mod prompt;
mod commandler;
mod get_path;

// Imports
use rustyline::Editor;
use std::collections::HashMap;

// Main function
fn main(){
    // Read in prompt
    let prompt_and_read: (String, String) = prompt::get_prompt();
    let read_prompt = prompt_and_read.clone().1;
    // Get aliases
    let aliases: HashMap<String, String> = alias::get_aliases();
    // Create the reader
    let mut reader = Editor::<()>::new().unwrap();
    // Infinitely loop
    loop{
        // Regenerate prompt
        let prompt = prompt::prompt_gen(read_prompt.clone());
        // Read line
        let readline = alias::alias_swap(reader.readline(prompt.as_str()).unwrap(), aliases.clone());
        // Pass line to command handler
        commandler::commandler(readline);
    }
}
