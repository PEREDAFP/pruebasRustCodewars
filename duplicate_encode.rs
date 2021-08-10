///Duplicate Encode

//The goal of this exercise is to convert a string to a new string where each character in
//the new string is "(" if that character appears only once in the original string, or ")"
//if that character appears more than once in the original string.
//Ignore capitalization when determining if a character is a duplicate.

//Examples

//"din"      =>  "((("
//"recede"   =>  "()()()"
//"Success"  =>  ")())())"
//"(( @"     =>  "))(("

//Notes

//Assertion messages may be unclear about what they display in some languages. If you read "...It Should encode XXX", the "XXX" is the expected result, not the input!
fn duplicate_encode(word:&str)->String {
    use std::collections::HashMap;
    let mut result = String::new();
    let mut letras = HashMap::new();

    for letra in word.chars(){
        let veces = letras.entry(letra).or_insert(0);
        *veces += 1;
    }
    for letra in word.chars(){
        match letras.get(&letra){
            Some(i)=>if *i == 1 {
                        result.push('(');
                    }else {
                        result.push(')');
                    },
            None=> result.push(' '),
        };
    }
    result

}
fn main() {

    println!("{}", duplicate_encode("din"));
    println!("{}", duplicate_encode("recede"));
    println!("{}", duplicate_encode("((ñ@"));
}
