// Imports
use std::env::current_dir;
use std::process::Command;
use std::str::from_utf8;
use std::fs::File;
use std::io::Read;

// Return current directory, either full or basename based on bool
#[allow(dead_code)]
pub fn get_current_dir(basename: bool) -> String{
    // Get full current directory
    let current_user_dir: String = current_dir().unwrap().to_str().expect("Current directory not able to be read").to_string();
    // If basename is wanted, just get basename
    if basename{
        let current_user_dir_basename_arr: Vec<String> = current_user_dir.split("/").map(|s| s.to_string()).collect();
        let current_user_dir_basename: String = current_user_dir_basename_arr[current_user_dir_basename_arr.clone().len() - 1].clone();
        return current_user_dir_basename;
    }
    // Return current dir
    return current_user_dir;
}

// Return username
#[allow(dead_code)]
pub fn get_username() -> String{
    // Get uname from system
    let mut uname_command = Command::new("whoami");
    // Convert to String
    let mut uname: String = from_utf8(&uname_command.output().unwrap().stdout[..]).unwrap().to_string();
    // Remove `\n` from username
    uname.pop();
    // Return uname
    return uname;
}

// Returns hostname
#[allow(dead_code)]
pub fn get_hostname() -> String{
    // Open `/etc/hostname`
    let mut etc_hostname = File::open("/etc/hostname").expect("/etc/hostname could not be opened");
    // Read `/etc/hostname` to String
    let mut hostname = String::new();
    etc_hostname.read_to_string(&mut hostname).expect("Unable to read /etc/hostname");
    // Remove `\n` from hostname
    hostname.pop();
    // Return hostname
    return hostname;
}
