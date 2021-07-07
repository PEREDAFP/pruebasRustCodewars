/// Array.Diff
// El objetivo es implementar la función array_diff que sustrae una lista de otra
// y devuelve el resultado
//Se deben eliminar todos los valores de la lista a que están presentes en la lista b
//
//array_diff(vec![1,2], vec![2]) == vec![1]

// Si un valor de a está presente en b, todas las ocurrencias deben ser eliminadas
//array_diff(vec![1,2,2,2,3], vec![2]) == vec![1,3]
fn array_diff<T: PartialEq>(a: Vec<T>, b:Vec<T>)-> Vec<T>{

    //Utilizar T nos va a permitir utilizar esta función con cualquier tipo
    //que implemente el trait PartialEq

    let mut resultado :Vec<T> = Vec::new(); //también puede hacerse con vec![]

    //Creamos un primer array únicamente con los elementos que aparecen en a y están en b
    let auxiliar: Vec<T> =a.into_iter()
    .filter(|s| {
        match b.iter().find(|&x| x == s){
            Some(_) => true,
            None => false,
        }
    })
    .collect();

    //Evitamos los duplicados.
    //todo!() --> sería interesante utilizar otro filter en auxiliar...
    for i in auxiliar{
        if resultado.iter().filter(|&n| *n == i).count() == 0 {
            resultado.push(i);
        }
    }
    //Devolvemos el array
    resultado
}

//Lo probamos
fn main(){
    let a =vec![1,2,3,4,5,6,6,3,1];
    for i in array_diff(a,vec![3]) {
        println!("{}",i);
    }
}
