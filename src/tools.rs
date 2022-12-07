// Get first word from String
#[allow(dead_code)]
pub fn seperate_first_substr(full_string: String) -> (String, String){
    // Initialize variables
    let mut space_found: bool = false;
    let mut starting_substring = "".to_owned();
    let mut ending_substring = "".to_owned();
    // Loop through
    for c in full_string.chars(){
        // If there hasn't been a space
        if !space_found{
            // If char isn't a space, add to first substring
            if c != ' '{
                starting_substring = format!("{}{}", starting_substring, c);
            }
            // If it was a space, make the space_found bool true
            else{
                space_found = true;
            }
        }
        // If there has been a space, add char to second substring
        else{
            ending_substring = format!("{}{}", ending_substring, c);
        }
    }
    // Create return tuple from starting and ending substrings
    let return_tuple: (String, String) = (starting_substring.clone(), ending_substring.clone());
    // Return return tuple
    return return_tuple;
}
