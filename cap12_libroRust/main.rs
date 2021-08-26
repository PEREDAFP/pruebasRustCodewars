use minigrep::Config;
fn main() {

    let config = match Config::new(std::env::args()){
        Ok(c)  => c,
        Err(e) => {
            println!("Se ha producido el error: {}", e);
            std::process::exit(1);
        }
    };
    println!("{:?}", config.run());
}
