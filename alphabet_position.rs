///Alphabet Position

//In this kata you are required to, given a string, replace every letter with its position in the alphabet.

//If anything in the text isn't a letter, ignore it and don't return it.

//"a" = 1, "b" = 2, etc.
//Example

//alphabet_position("The sunset sets at twelve o' clock.")

//Should return "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11" (as a string)

fn alphabet_position(text: &str) -> String {
    const BASE:u32 = 96; //La letra a es el número 97...entonces...
    let mut result = String::new(); //Creamos el String que devolveremos

    for c in text.to_lowercase().chars(){
        if c.is_alphabetic(){
            result.push_str(&(c as u32 - BASE).to_string());
            result.push(' ');
        }
    }
    result.trim().to_string() //Hacemos el trim para eliminar el último espacio en blanco
}

fn alphabet_position2(text: &str) -> String {
//Refactorizamos y...

    const BASE:u32 = 96; //La letra a es el número 97...entonces...
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c as u32 - BASE).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    println!("{}", alphabet_position("The sunset sets at twelve o' clock."));
    println!("{}", alphabet_position2("The sunset sets at twelve o' clock."));
}
