/*
Convert strings to pig latin.
The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
*/

pub fn strings_to_pig_latin(word: &str) -> String {
    let vowels: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
    let first_char = &word[0..1];
    let mut is_vowel: bool = false;

    for char in vowels.iter() {
        if first_char as char == *char {
            is_vowel = true;
            break;
        }
    }

    if is_vowel {
        return word.to_string() + "-hay";
    } else {
        return word[1..].to_string() + "-" + &first_char + "ay";
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings_to_pig_latin() {
        assert_eq!(strings_to_pig_latin("first"), "irst-fay");
        assert_eq!(strings_to_pig_latin("apple"), "apple-hay");
    }
}
