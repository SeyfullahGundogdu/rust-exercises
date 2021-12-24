fn main() {
    //tests
    assert_eq!("irst-fay",pig_latin(&"first"));
    assert_eq!("apple-hay",pig_latin(&"apple"));
}

/// Convert strings to pig latin.
/// 
/// The first consonant of each word is moved to the end of the word and “ay” is added.
///  Words that start with a vowel have “hay” added to the end instead.
fn pig_latin(input: &str) -> String {
    
    let mut chars = (*input).chars();
    let mut new_str = String::new();
    let mut count = 0;
    while let Some(c) = chars.next() {
        // check if the first index has a vowel or a consonant.
        // if not, skip to the next iteration without pushing the value to new_str
        // even though char might never meet the first two conditions; rust makes suffix of type String
        // because if the first comparison might evaluate to String
        let suffix = match c {
            'a'| 'e'| 'i'|  'o'| 'u'| 'A'| 'E'| 'I'|  'O'| 'U' => {
                println!("first letter is a vowel");
                new_str.push(c);
                format!("-hay")
            },
            'a'..='z' | 'A'..='Z' => {
                println!("first letter is a consonant");
                format!("-{}ay",c)
            },
            _ => {
                println!("no alphanumeric char found at index {}", count);
                count += 1;
                continue;
            },
        };
        //consume the next char from the String and push it to the new_str repeatedly.
        //code will not reach this line if it doesn't contain any alphanumeric values.
        //therefore returned string will be empty.
        while let Some(c) = chars.next() {
            new_str.push(c); 
        }
        //append the suffix to the piece of string that we pushed to new_str
        new_str.push_str(&suffix);
    }
    //return new_str
    new_str
}
