/*
Kata 1 de codewars.com
Realizada: 24-06-2021

Nos pasan un texto como el siguiente:
"
1000.00!=

125 Market !=:125.45
126 Hardware =34.95
127 Video! 7.45
128 Book :14.32
129 Gasoline ::16.10
"
En la primer línea tenemos la cantidad inicial
En las siguientes, el primer número indica el cheque utilizado, a continuación el concepto y luego
el precio

Debemos devolver una cadena de esta forma:
"Original Balance: 1000.00
125 Market 125.45 Balance 874.55
126 Hardware 34.95 Balance 839.60
127 Video 7.45 Balance 832.15
128 Book 14.32 Balance 817.83
129 Gasoline 16.10 Balance 801.73
Total expense  198.27
Average expense  39.65"

Nos hemos encontrado con las siguientes dificultades:
1.- La precisión de los floats a sólo dos decimales. Solucionado con &format!("{:.2}", total)
2.- Tener que trabajar con .iter() en el vector. Creía que con realizar el collect
    ya se hacía un elemento iterable...cosillas
*/

fn balance(book: &str)  -> String{

    let mut total = 0.0_f32;
    let mut gasto_acumulado = 0.0_f32;
    let mut nuevo = String::new();
    let mut devuelto = String::new();
    let mut compras: i32 = 0;

    //Primera pasada para eliminar caracteres indeseables...
    for c in book.trim().chars(){
        if c.is_alphabetic() || c.is_numeric() || c == ' ' || c == '.' || c== '\n'{
            nuevo.push(c);
        }
    }
    //Pasamos la cadena a vector, mediante shadowing, para poder trabajar con los elementos
    //que forman parte de cada cadena
    let nuevo: Vec<&str> = nuevo.split('\n').collect();

    //Vamos a utilizar linea para tratar de manera diferente la primera y el resto
    for (linea ,cadena) in nuevo.iter().enumerate() {
        if cadena.trim().len() > 0 {
            if linea == 0 {
                total = cadena.trim().parse::<f32>().unwrap();
                devuelto.push_str(&format!("Original Balance: {}",cadena.trim()));

            }
            else {
                //Componemos la cadena de cada una de las compras
                let auxiliar:Vec<&str> = cadena.split(' ').collect();
                let gasto = auxiliar[2].parse::<f32>().unwrap();

                devuelto.push_str(&format!("{} {} {} Balance {}",auxiliar[0], auxiliar[1], auxiliar[2],format!("{:.2}", total-gasto)));
                //Vamos calculando el gasto acumulado e incrementamos las compras para la media final
                gasto_acumulado += gasto;
                compras += 1;
            }
            devuelto.push('\n');
        }
    }
    //Ahora hacemos el total y la media
    devuelto.push_str(&format!("Total expense  {}\n" ,format!("{:.2}", gasto_acumulado)));
    devuelto.push_str(&format!("Average expense  {}\n" ,format!("{:.2}", gasto_acumulado/compras as f32)));

    //Devolvemos
    devuelto

}

fn main(){
    let lista="
1233.00
125 Hardware;! 24.8?;
123 Flowers 93.5
127 Meat 120.90
120 Picture 34.00
124 Gasoline 11.00
123 Photos;! 71.4?;
122 Picture 93.5
132 Tyres;! 19.00,?;
129 Stamps 13.6
129 Fruits{} 17.6
129 Market;! 128.00?;
121 Gasoline;! 13.6?;
";
    let resul = balance(lista);
    println!("{}", resul);
}
