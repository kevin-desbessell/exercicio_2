// Neste código temos que pegar uma fase em, sempre que uma palaavra começar com vogal, adicionar "hay" no final da palavra, como exemplo "apple" -> "apple-hay", e para consoantes temos que adicionar a primeira letra da palavra no final da palavra e adicionar "ay", como exemplo "banana" -> "anana-bay".

fn main() {
    let string = "once upon a time there was a Hobbit named Frodo Baggins and he went to Mordor to destroy the One Ring";

    let mut result = String::new();
    for word in string.split_whitespace() {
        let mut word = word.to_string();
        let first_letter = word.chars().next().unwrap();
        if "aeiou".contains(first_letter) {
            word.push_str("-hay");
        } else {
            word.push_str(&format!("-{}ay", first_letter));
            word.remove(0);
        }
        result.push_str(&format!("{} ", word));
    }

    println!("{}", result);
}