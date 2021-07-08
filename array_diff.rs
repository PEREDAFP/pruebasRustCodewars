/// Array.Diff
// El objetivo es implementar la función array_diff que sustrae una lista de otra
// y devuelve el resultado
//Se deben eliminar todos los valores de la lista a que están presentes en la lista b
//
//array_diff(vec![1,2], vec![2]) == vec![1]

// Si un valor de a está presente en b, todas las ocurrencias deben ser eliminadas
//array_diff(vec![1,2,2,2,3], vec![2]) == vec![1,3]
fn array_diff<T: PartialEq + Ord>(a: Vec<T>, b: Vec<T>)-> Vec<T>{

    //Utilizar T nos va a permitir utilizar esta función con cualquier tipo
    //que implemente el trait PartialEq, si queremos utilizar .sort() es necesario que el Trait Ord también
    //sea parte del tipo con el que queremos trabajar

    //Creamos un primer array únicamente con los elementos que aparecen en a y están en b
    let mut auxiliar: Vec<T> =  a.into_iter()
    .filter(|s| {
        match b.iter().find(|&x| x == s){
            Some(_) => true,
            None => false,
        } //Tenemos que devolver un bool para que el elemento sea filtrado o no
    })
    .collect();
    //Faltaría por investigar por qué no es posible realizar todo lo que sigue enlazando con un .
    //Y evitándonos tener que realizar una variable nueva
    auxiliar.sort();   //Es necesario para que dedup pueda hacer su magia
    auxiliar.dedup();  //Elimina los elementos duplicados dejando un único representante


    auxiliar
    }

//Lo probamos
fn main(){
    let a =vec![1,2,3,4,5,6,6,3,1];
    for i in array_diff(a,vec![8,9,10]) {
        println!("{}",i);
    }
    //Sería interesante retocar la función para no hacer un préstamo del vec!
    // y permitir cosas como esta: println!("{:?}", a);
}
