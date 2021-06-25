/*
Los cajeros automáticos permiten códigos PIN de 4 o 6 dígitos
y los códigos PIN no pueden contener más que exactamente 4 dígitos o exactamente 6 dígitos.

Si a la función se le pasa una cadena de PIN válida,
devuelve true, de lo contrario devuelve false.

"1234"   -->  true
"12345"  -->  false
"a234"   -->  false


CHUPAOOO
*/
fn validate_pin(pin: &str) -> bool {
    for c in pin.trim().chars(){
        if  !(c.is_numeric()) {
            return false;
        }
    }
    pin.len() == 4 || pin.len() == 6
}
fn main(){
    println!("{}", validate_pin("123456"));
    println!("{}", validate_pin("a23456"));
    println!("{}", validate_pin("1234"));
    println!("{}", validate_pin("56"));



}
