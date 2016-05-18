#[macro_use]
extern crate clap;
extern crate csv;

type Row = (f64, f64);

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn write_to_file(result: &String, filename: &String)
{
    let path = Path::new(filename);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    if let Err(_) = std::fs::metadata(path.parent().unwrap()){
        std::fs::create_dir(path.parent().unwrap()).unwrap()
    }
    if let Err(why) = file.write_all(result.as_bytes()) {
        panic!("Could not write to {}: {}", display, Error::description(&why));
    }
}

fn linear_regression(input_file: String)
{
    let mut rdr = csv::Reader::from_file(input_file).unwrap();
    let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();
    let lr: f64 = 0.07;
    let min = rows.iter().map(|&(km, _)| km as i32).min().unwrap() as f64;
    let max = rows.iter().map(|&(km, _)| km as i32).max().unwrap() as f64;
    let rescale = |val| (val - min) / (max - min);
    let estimate_price = |mileage, (x, y)| x + (y * mileage);
    let (t0, t1) = (0..60000).fold((0.0, 0.0), |(x, y), _| {
        let res: (f64, f64) = rows.iter().fold((0.0, 0.0), |(x1, x2), &(km, price)| {
            (x1 + (estimate_price(rescale(km), (x, y)) - price), x2 + ((estimate_price(rescale(km), (x, y)) - price) * rescale(km)))
        });
        (x - (res.0 / rows.len() as f64 * lr), y - (res.1 / rows.len() as f64 * lr))
    });
    write_to_file(&format!("{},{}\n{},{}", t0, t1, min, max), &String::from("/tmp/afaucher/.tmp"));
}

fn main() {
   let yaml = load_yaml!("cli.yml");
   let options = clap::App::from_yaml(yaml).get_matches();
   linear_regression(String::from(options.value_of("file").unwrap()));
}
