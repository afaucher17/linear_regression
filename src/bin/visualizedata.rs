#[macro_use]
extern crate clap;
extern crate csv;

type Row = (f64, f64);

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

fn linear_regression(input_file: String)
{
    let mut rdr = csv::Reader::from_file(input_file).unwrap();
    let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();
    let lr: f64 = 0.07;
    let min = rows.iter().map(|&(km, _)| km as i32).min().unwrap();
    let max = rows.iter().map(|&(km, _)| km as i32).max().unwrap();
    let diff = (max - min) as f64;
    let rescale = |x| (x - min as f64) / diff;
    let estimate_price = |mileage, (x, y)| x + (y * rescale(mileage));
    let (t0, t1) = (0..3000).fold((0.0, 0.0), |(x, y), _| {
        let res: (f64, f64) = rows.iter().fold((x, y), | _, &(km, price)| {
            (estimate_price(km, (x, y)) - price, (estimate_price(km, (x, y)) - price) * rescale(km))
        });
        (x - (res.0 / rows.len() as f64 * lr), y - (res.1 / rows.len() as f64 * lr))
    });
    for &(km, price) in rows.iter()
    {
        println!("{} {}", price, estimate_price(km, (t0, t1)));
    }
}

fn main() {
   let yaml = load_yaml!("cli.yml");
   let options = clap::App::from_yaml(yaml).get_matches();
   linear_regression(String::from(options.value_of("file").unwrap()));
}
