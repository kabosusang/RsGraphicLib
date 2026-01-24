use std::fs;
use std::io;


use graphiclib::EnvConfig;

fn read_binrary_file(path: &str) -> io::Result<Vec<u8>> {
    let data = fs::read(path)?;
    Ok(data)
}

fn main() {
    let bin_vec = read_binrary_file("fnm.png");

    match bin_vec {
        Ok(data) => {
            let first: u8 = data[0];
            println!("First Char: {0}", first);
        }
        Err(e) => {
            eprintln!("No Found File Path: {0}", e);
        }
    }
    if let Ok(config) = EnvConfig::<EnvShell>::build() {
        println!("{:#?}", config);
    }

	
}
