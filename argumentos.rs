///Programa que toma argumentos desde la línea de comandos
//Probamos también la función swap para itercambio de valores
//entre variables sin utilizar una tercera variable.

use std::str::FromStr;
use std::env;
use std::mem;

fn gcd(mut n :u64, mut m: u64) -> u64 {
    if n == 0  || m == 0    {
        return 0
    }
    while m != 0 {
        if m < n {
            //Hacemos el intercambio de los valores
            //sería similar a un m,n = n,m en python
            mem::swap(&mut m, &mut n);


            /*Podríamos haberlo realizado así:
            m = m + n;
            n = m - n;
            m = m - n;
            */
        }
        m = m % n;
    }
    n
}

fn main(){
    let mut numbers = Vec::new();
    //Nos saltamos el primer valor que será el nombre del programa
    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg)
            .expect("error de argumento..."));
    }
    if numbers.len() == 0 {
        eprintln!("!Ojo: gcd NÚMERO NÚMERO...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d,*m);
    }
    println!("El gcd de {:?} es {}", numbers, d);
}
