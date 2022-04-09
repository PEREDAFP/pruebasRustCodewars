//https://taller-1-fiuba-rust.github.io/Inicio.html
//Resolución del primer ejercicio de la Guía 3

use std::thread;
use std::sync::{Arc,Condvar,Mutex};
//Solo se puede utilizar Condvar con Mutex, por lo que descartamos la más eficiente
//RwLock

struct Account(Mutex<i32>, Condvar);

impl Account {
    fn deposit(&self, amount: i32) {
        let mut guard = self.0.lock().unwrap();
        println!("op: deposit {}, available funds: {:?}", amount, *guard);
        *guard += amount;
        self.1.notify_one();
    }

    fn withdraw(&self, amount: i32) {
        let mut guard = self.0.lock().unwrap();
        println!("op: withdraw {}, available funds: {}", amount, *guard);
        /* Esto ya no es necesario. El hilo se bloqueará hasta que la cantidad a
        descontar sea inferior a la cantidad que posee el struct
        if  *guard >= amount {
            *guard -= amount;
        } else {
            panic!("Error: Insufficient funds.")
        }*/
        while amount > *guard {
            guard = self.1.wait(guard).expect("Can't wait");
        }

        *guard -= amount;
    }

    fn balance(&self) -> i32 {
        let guard = self.0.lock().unwrap();
        *guard
    }
}

//static ACCOUNT: Arc<Account> = Arc::new(Account(Mutex::new(0),Condvar::new()));
fn main() {
    //No he podido una static de ACCOUNT...a investigar
    //Ojo con crear un Arc mut aunque parezca lo oportuno
    //El Mutex o el RwLock,en su caso, son los que se encargan de proporcionar
    //Esa mutabilidad.

    let ACCOUNT = Arc::new(Account(Mutex::new(0),Condvar::new()));
    let deposito1 = Arc::clone(&ACCOUNT);

    let customer1_handle = thread::spawn(move || {
        deposito1.deposit(40);
    });

    let deposito2 = Arc::clone(&ACCOUNT);
    let customer2_handle = thread::spawn(move ||  {
        deposito2.withdraw(30);
    });

    let deposito3 = Arc::clone(&ACCOUNT);
    let customer3_handle = thread::spawn(move || {
        deposito3.deposit(60);
    });

    let deposito4 = Arc::clone(&ACCOUNT);
    let customer4_handle = thread::spawn(move || {
        deposito4.withdraw(170);
    });
    let handles = vec![
    customer1_handle,
    customer2_handle,
    customer3_handle,
    customer4_handle,
    ];

    for handle in handles {
        handle.join().unwrap();
    }

    let savings = ACCOUNT.balance() ;

    println!("Balance: {}", savings);
}
