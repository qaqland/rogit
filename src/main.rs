mod config;
mod database;
mod update;
use config::Mode;

fn main() {
    let mut c = config::Config::new().unwrap();
    c.scan();
    println!("{c}");

    let r = match c.mode {
        Mode::Update => update::run(&c),
        Mode::Server => update::run(&c),
    };
    match r {
        Ok(_) => println!("Done!"),
        Err(e) => eprintln!("{}", e),
    }
}
