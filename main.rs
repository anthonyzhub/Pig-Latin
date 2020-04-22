/* NOTE:
* push_str -> Add string
* push -> Add character
*/

fn translate_string(curr_str: &mut String) -> String
{
    // Create blank string
    let mut new_str = String::new();

    // Iterate string
    for letter in curr_str.chars()
    {
        // Use match operator to check how to modify string
        match letter
        {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // Add current string to new string and append "-hay" to it
                new_str.push_str(curr_str);
                new_str.push_str("-hay");
            },
            _ => {
                // Add current string to new string and remove the 1st letter
                new_str.push_str(curr_str);
                new_str.remove(0);

                // Append -{letter}ay
                new_str.push('-');
                new_str.push(letter);
                new_str.push_str("ay");
            },
        }

        // Exit for-loop
        break;
    }

    // Return new string
    new_str
}

fn main() {
    
    // Test strings
    let mut str_1 = String::from("apple");
    let mut str_2 = String::from("first");

    println!("{}", translate_string(&mut str_1));
    println!("{}", translate_string(&mut str_2));
}
