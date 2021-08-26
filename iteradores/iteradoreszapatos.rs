//Mejora del ejemplo que aparece en el libro de Rust capítulo 13 ya que al crear una función
//que "consume" el iterador, el resto de elementos queda inaccesible

//El aporte es de @Categulario

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_my_size(shoes: &Vec<Shoe>, shoe_size: u32) -> impl Iterator<Item=&Shoe>{
            shoes.iter()
            .filter(move |s| s.size == shoe_size)
    }


fn main() {
        let mut shoes = vec![
        Shoe {size:10, style: String::from("sneaker 10")},
        Shoe {size:12, style: String::from("sneaker 12")},
        Shoe {size:10, style: String::from("sneaker 10.5")},
        Shoe {size:11, style: String::from("sneaker 11")},
        ];

        let in_my_size = shoes_in_my_size(&shoes, 10);
        for shoe in in_my_size {
            println!("{:?}", shoe);

        }
