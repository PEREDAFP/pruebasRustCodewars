/*
Write a function that takes in a string of one or more words, and returns the same string, but with all five or more letter words reversed (like the name of this kata).

    Strings passed in will consist of only letters and spaces.
    Spaces will be included only when more than one word is present.

Examples:

spinWords("Hey fellow warriors") => "Hey wollef sroirraw"
spinWords("This is a test") => "This is a test"
spinWords("This is another test") => "This is rehtona test"

*/

fn spin_words(words: &str) -> String {
    let mut result = Vec::new();
    for i in words.split_whitespace() {
        if i.len() > 5{
            //Invertimos la cadena
            //realizamos un chars, que recorremos a la inversa y realizamos un collect de String
            result.push(i.chars().rev().collect::<String>() );
        }else {
            result.push(i.to_string());
        }
     }
     //Convertimos el Vec de String a un String con el método join
    result.join(" ")
}

fn main(){
    let cadena = "hola mundo traidor";
    println!("{}", spin_words(&cadena));
}
