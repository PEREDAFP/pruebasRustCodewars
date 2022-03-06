/*
https://www.codewars.com/kata/5e0b72d2d772160011133654/train/rust

Te dan tres montones de fichas de casino: fichas blancas, verdes y negras:

    el primer montón contiene sólo fichas blancas
    el segundo montón contiene sólo fichas verdes
    el tercer montón contiene sólo fichas negras

Cada día coges exactamente dos fichas de diferentes colores y te diriges al casino. Puedes elegir cualquier color, pero no puedes coger dos fichas del mismo color en un día.

Se te dará una matriz que representa el número de fichas de cada color y tu tarea es devolver el número máximo de días que puedes coger las fichas. Cada día tienes que coger exactamente dos fichas.

Examples (input -> output)

* [1,1,1] -> 1, because after you pick on day one, there will be only one chip left
* [1,2,1] -> 2, you can pick twice; you pick two chips on day one then on day two
* [4,1,1] -> 2*/

fn solve(arr: &[u32; 3]) -> u32 {
   /*return math.floor(min( a+b, b+c, c+a, math.floor(a+b+c)/2 ))*/
  
   match [arr[0]+ arr[1], arr[1]+ arr[2], arr[0] + arr[2], (arr[0]+arr[1]+arr[2])/2 ].iter().min() {
       None => 0u32,
       Some(i) => *i,
   }
}

fn main() {
   
    println!("{}", solve(&[1,1,1]));
}
