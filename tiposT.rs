///Vamos a probar qué ocurre si definimos un struct con métodos con el
///mismo nombre pero distinto tipo

//Esto no es un ejercicio de CodeWars

fn main() {

    //Definimos el struct con tipo genérico
    //Si los tipos de las propiedades del struct fueran diferentes habría
    //que realizar un <T,U> y ya está.
    struct Point<T>{
        x: T,
        y: T,
    }

    //Ponemos un ejemplo de función asociada y de métodos con tipos genéricos
    impl <T> Point<T>{
    fn new(valorx:T,valory:T)->Point<T>{
            Point{
                x:valorx,
                y:valory,
            }
    }
    fn devuelve_x(&self)-> &T{
        &self.x
    }
    fn devuelve_y(&self)-> &T{
        &self.y
    }
 }
    //Ahora nos centramos en uno de los tipos
    //Debemos observar que después de impl no se indica el tipo
    impl Point<f32>{
        fn depende_tipo(&self)->String{
            "esto es f32".to_string()
        }
    }

    //Ahora nos centramos en uno de los tipos
    //Debemos observar que después de impl no se indica el tipo

    impl Point<u32>{
        fn depende_tipo(&self)->String{
            "esto es u32".to_string()
        }
    }

    let flotante = Point::new(3.0,4.3);
    let entero = Point::new(1,2);
    println!("{}", flotante.depende_tipo());
    println!("{}", entero.depende_tipo());
    println!("{}",entero.devuelve_x());
    println!("{}",flotante.devuelve_y());
}
