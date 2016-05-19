extern crate csv;

type Row = (f64, f64);

use std::io;

fn main () {
    let (t0, t1, min, max): (f64, f64, f64, f64) = match std::fs::metadata("/tmp/afaucher/.tmp") {
        Err(_) => (0.0, 0.0, 0.0, 0.0),
        Ok(_) => { 
            let mut rdr = csv::Reader::from_file("/tmp/afaucher/.tmp").unwrap().has_headers(false);
            let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();
            (rows[0].0, rows[0].1, rows[1].0, rows[1].1)
        }
    };
    println!("Please enter a mileage: ");
    let mut mileage = String::new();
    io::stdin().read_line(&mut mileage).expect("failed to read the mileage");
    let mileage: f64 = mileage.trim().parse().expect("The mileage you entered is not an interger.");
    println!("Estimated price: {0:.2}", t0 + (t1 * if min != 0.0 && max != 0.0 { (mileage - min) / (max - min) } else { mileage }));
}
