#[macro_use]
extern crate clap;
extern crate csv;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn read_file(filename: String) -> String {
    let path = Path::new(&filename);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("Could not read {}: {}", display, Error::description(&why)),
        Ok(_) => buffer,
    }
}

fn main() {
   let yaml = load_yaml!("cli.yml");
   let options = clap::App::from_yaml(yaml).get_matches();
   let data = read_file(String::from(options.value_of("file").unwrap()));
   let mut rdr = csv::Reader::from_string(data);
   for row in rdr.records().map(|r| r.unwrap()) {
       println!("{:?}", row);
   }
}
