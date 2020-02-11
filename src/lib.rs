#[derive(Debug, Clone)]
pub struct BFProgram {
  pub tape: Vec<i32>,
  pub position: i32,
}

impl BFProgram {

  pub fn plus(self: &BFProgram) -> () {
    println!("Added")
  }

  pub fn minus(self: &BFProgram) -> () {
    println!("Subtracted")
  }
  
  pub fn shift_left(self: &BFProgram) -> () {
    println!("Shifted to left")
  }

  pub fn shift_right(self: &BFProgram) -> () {
    println!("Shifted to left")
  }
}
