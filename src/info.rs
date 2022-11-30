use std::env::current_dir;
use std::process::Command;
use std::str::from_utf8;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
pub fn get_current_dir(basename: bool) -> String{
    let current_user_dir: String = current_dir().unwrap().to_str().expect("Current directory not able to be read").to_string();
    if basename{
        let current_user_dir_basename_arr: Vec<String> = current_user_dir.split("/").map(|s| s.to_string()).collect();
        let current_user_dir_basename: String = current_user_dir_basename_arr[current_user_dir_basename_arr.clone().len() - 1].clone();
        return current_user_dir_basename;
    }
    return current_user_dir;
}
#[allow(dead_code)]
pub fn get_username() -> String{
    let mut uname_command = Command::new("whoami");
    let mut uname: String = from_utf8(&uname_command.output().unwrap().stdout[..]).unwrap().to_string();
    uname.pop();
    return uname;
}
#[allow(dead_code)]
pub fn get_hostname() -> String{
    let mut etc_hostname = File::open("/etc/hostname").expect("/etc/hostname could not be opened");
    let mut hostname = String::new();
    etc_hostname.read_to_string(&mut hostname).expect("Unable to read /etc/hostname");
    hostname.pop();
    return hostname;
}
