fn main() {
    let test = "Hello world";
    let convert = convert_to_pig_latin(&test);
    
    println!("{} converted to pig latin: {}", test, convert);
}

fn convert_to_pig_latin(original: &str) -> String {
    let vowels = ["a", "e", "i", "o", "u"];
    let mut converted = String::new();

    for word in original.split(' ') {
        let first_letter = &word[0..1];

        if vowels.contains(&first_letter) {
            converted = converted + word + "hay "
        };
    
        let rest = &word[1..word.len()];
        converted = converted + rest + &first_letter.to_lowercase() + "ay ";
    }

    converted
}