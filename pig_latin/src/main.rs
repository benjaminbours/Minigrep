use std::io;

const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    println!("Enter a sentence to convert it to pig latin");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input_pig_latin = convert_to_pig_latin(&user_input.trim());

    println!("{}", user_input_pig_latin);
}

fn convert_to_pig_latin(input: &str) -> String {
    let words: Vec<&str> = input.split(" ").collect();
    let mut pig_latin_words: Vec<String> = Vec::new();

    for &single_word in &words {
        let first_char = single_word.chars().next().unwrap();
        let new_word: String;

        if VOWEL.contains(&first_char) {
            new_word = format!("{}-hay", single_word);
        } else {
            let rest_of_word = &single_word[1..];
            new_word = format!("{}-{}ay", rest_of_word, first_char);
        }
        pig_latin_words.push(new_word);
    }

    pig_latin_words.join(" ")
}
