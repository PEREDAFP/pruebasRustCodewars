//Categorize New Member
/*
The Western Suburbs Croquet Club has two categories of membership, Senior and Open. They would like your help with an application form that will tell prospective members which category they will be placed.

To be a senior, a member must be at least 55 years old and have a handicap greater than 7. In this croquet club, handicaps range from -2 to +26; the better the player the lower the handicap.
Input

Input will consist of a list of lists containing two items each. Each list contains information for a single potential member. Information consists of an integer for the person's age and an integer for the person's handicap.

Note for F#: The input will be of (int list list) which is a List<List>
Example Input

vec![(45, 12), (55,21), (19, -2), (104, 20)])

Output

Output will consist of a list of string values (in Haskell: Open or Senior) stating whether the respective member is to be placed in the senior or open category.
Example Output

vec!["Open", "Senior", "Open", "Senior"]
*/

//Aunque en la definición del problema en codewars aparecía data: Vec, he realizado las modificaciones
//oportunas para no "apropiar" el vector por parte de la función con data: &Vec
//se han utilizado los dereferenciadores (*) donde ha sido necesario.
fn open_or_senior(data: &Vec<(i32, i32)>) -> Vec<String> {
  let mut result = Vec::<String>::new();
  for jugador in data{
      if jugador.0 < 55 {
          result.push("Open".to_string());
      }else if jugador.1 > 7{
          result.push("Senior".to_string());
      }else {
          result.push("Open".to_string());
      }
  }
    result
}

fn open_or_senior2(data: &Vec<(i32,i32)>) -> Vec<String>{
    const EDAD_SENIOR:i32 = 55;
    const HANDICAP_MINIMO: i32 = 8;

    const SENIOR: &str= "Senior";
    const OPEN: &str = "Open";

    data.into_iter()
    .map(|(edad, handicap)| {
        if *edad >= EDAD_SENIOR && *handicap >= HANDICAP_MINIMO {
            SENIOR
        }else {
            OPEN
        }
    })
    .map(String::from)
    .collect::<Vec<String>>()
}

fn open_or_senior3(data: &Vec<(i32, i32)>) -> Vec<String>{
    const EDAD_SENIOR:i32 = 55;
    const HANDICAP_MINIMO: i32 = 8;

    const SENIOR:&str= "Senior";
    const OPEN:&str = "Open";
    data.iter()
    .map(|&(x,y)| match (x> EDAD_SENIOR, y >= HANDICAP_MINIMO) {
        (true, true) => SENIOR,
        _ => OPEN
    })
    .map(String::from)
    .collect()
}
fn main(){
    let jugadores = vec![(45, 12), (55,21), (19, -2), (55, 6)];
    println!("{:?}", open_or_senior3(&jugadores));
    //Sin el cambio &Vec la siguiente línea no permitiría compilar.
    //Realiza la prueba y explica el porqué
    println!("{:?}", jugadores);
}
