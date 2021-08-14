// Convert strings to pig latin. The first consonant of each word is moved to the end of the word
// and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
// to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
// encoding!

fn main() {
    let mut word = String::from("apple");

    let first_letter_is_consonant = is_consonant(&word[0..1]);

    if first_letter_is_consonant == true {
        word = format!("{}-{}ay", &word[1..word.len()], &word[0..1]);
    } else {
        word = format!("{}-hay", word);
    }

    print!("{}", word);
}

fn is_consonant(letter: &str) -> bool {
    match letter {
        "a" => false,
        "A" => false,
        "e" => false,
        "E" => false,
        "i" => false,
        "I" => false,
        "o" => false,
        "O" => false,
        "u" => false,
        "U" => false,
        _ => true,
    }
}
