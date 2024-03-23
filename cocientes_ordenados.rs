/*
Puedes ver el enunciado en:
https://www.codewars.com/kata/569f6ad962ff1dd52f00000d/train/rust
*/  
fn select_quotients(arr: &[u32], m: u32, dir_str: &str) -> Vec<(u32, (u32, u32))> {
    use std::cmp::Ordering;
    
    let mut resultado: Vec<(u32, (u32, u32))>=Vec::new();
    for (ind,a) in arr.iter().enumerate(){
        for b in ind+1..arr.len(){
            let mayor:u32;
            let menor:u32;
            match a.cmp(&arr[b]) {
                Ordering::Less => {
                    mayor = arr[b];
                    menor = *a;
                },
                Ordering::Greater =>{
                    mayor = *a;
                    menor = arr[b];
                },
                Ordering::Equal => {
                    mayor = arr[b];
                    menor = *a;
                },
            }
            if mayor % menor == 0 && mayor / menor >= m {
                let mut graba = false;
                if (dir_str == "Odd" || dir_str == "odd" || dir_str == "") && (mayor/menor) % 2 == 1{
                    graba = true;
                }else if (dir_str == "even" || dir_str== "Even"|| dir_str== "") && (mayor/menor) % 2 == 0  {
                    graba = true;
                }
                if graba {
                    resultado.push((mayor / menor,(mayor, menor)));
                }
                
            }
        }
    }
    if resultado.len() > 0 {/*
        Si utilizamos sort_by o similares el compilador indica que el vector puede estar vacío. Si se utiliza sort sin más, no hay problema
        resultado.sort_by(|a, b| {
            let cmp = a.0.cmp(&b.0); // Compara los cocientes
            if cmp == std::cmp::Ordering::Equal {
                a.1.cmp(&b.1) // Si los cocientes son iguales, compara los pares de números
            } else {
                cmp
            }
        });*/
        resultado.sort();
        resultado.dedup();
    }
    resultado
    

    }

fn main(){
    let arr = [2, 4, 27, 16, 9, 15, 25, 6, 12, 83, 24, 49, 7, 5, 94, 12, 6];
    println!("{:?}",select_quotients(&arr,2,""));

}
/* 
fn divisiones_enteras(nums: &[u32]) -> Vec<(u32, (u32, u32))> {
    let mut result = Vec::new();

    for (i, &num1) in nums.iter().enumerate() {
        for (j, &num2) in nums.iter().enumerate() {
            if i != j {
                let cociente = num1 / num2;
                let resto = num1 % num2;
                result.push((cociente, (num1, num2)));
            }
        }
    }

    result.sort_by_key(|&(cociente, _)| cociente);
    result
}

fn main() {
    let numeros = [10, 2, 5]; // Ejemplo de números de entrada
    let resultado = divisiones_enteras(&numeros);
    println!("{:?}", resultado);
}


*/
