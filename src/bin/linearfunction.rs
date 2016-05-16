use std::io;

fn main () {
  println!("Please enter a mileage: ");
  let mut mileage = String::new();
  io::stdin().read_line(&mut mileage).expect("failed to read the mileage");
  let mileage: u32 = mileage.trim().parse().expect("The mileage you entered is not an interger.");
  {
      println!("{}", mileage);
  }
}
