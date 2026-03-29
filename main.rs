use std::{
    env, fs,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    println!("Hello Selamat Datang di kernel lisan!\nSilahkan Lihat dokumentasinya di |veoveneht.eu.org|");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let intrim = input.trim();
        if intrim.starts_with("crd") {
            let name_dr = &intrim[4..].trim();

            let current_dr = env::current_dir()?;
            let new_path = current_dr.join(name_dr);

            fs::create_dir(new_path)?;

            println!("> Directory {} Created!", name_dr);
        } else if intrim.starts_with("dd") {
            let name_dr = &intrim[3..].trim();
            fs::remove_dir(name_dr)?;
            println!("{} deleted", name_dr);
        } else {
            println!("> Invalid Command");
        }
    }
}
