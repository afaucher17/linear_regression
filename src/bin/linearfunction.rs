#[macro_use]
extern crate csv;

type Row = (f64, f64);

use std::io;

fn main () {
    let (t0, t1, min, max) = if let Err(_) = std::fs::metadata("/tmp/afaucher/.tmp"){
        (0, 0, 0, 0)
    }
    else {
        let mut rdr = csv::Reader::from_file("/tmp/afaucher/.tmp").unwrap();
        let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();
        (rows[0].0, rows[0].1, rows[1].0, rows[1].1)
    };
    println!("Please enter a mileage: ");
    let mut mileage = String::new();
    io::stdin().read_line(&mut mileage).expect("failed to read the mileage");
    let mileage: u32 = mileage.trim().parse().expect("The mileage you entered is not an interger.");
    println!("{}", t0 + (t1 * if (min != 0 && max != 0) { mileage - min / (max - min) } else { mileage }));
}
