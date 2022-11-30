#[allow(dead_code)]
pub fn seperate_first_substr(full_string: String) -> (String, String){
    let mut space_found: bool = false;
    let mut starting_substring = "".to_owned();
    let mut ending_substring = "".to_owned();
    for c in full_string.chars(){
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
    let return_tuple: (String, String) = (starting_substring.clone(), ending_substring.clone());
    return return_tuple;
}
