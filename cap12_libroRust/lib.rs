use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    filename:String,
    contents:String,

}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        //Nos saltamos el argumento que indica el nombre del ejecutable
        args.next();

        //Nos aprovechamos del iterador que es args para realizar esta virguería...
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No hay segundo argumento"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No hay tercer argumento"),
        };
        //En el run original del libro se utiliza el ? al final del read_to_string
        //En el ejemplo del libro, el método run devuelve un Result<String, Box<dyn Error>>
        //He añadido el tratamiento de errores con un match para que devuelva lo mismo que los
        //dos args.next()
        let contents = match fs::read_to_string(&filename) {
            Ok(text) => text,
            Err(_) => return Err("Error con el fichero"),
        };

        Ok(Config {query, filename, contents})
    }

    pub fn run(&self) ->Vec<&str>{
        self.contents.lines()
            .filter(|a| a.contains(&self.query))
            .collect()
    }

}
