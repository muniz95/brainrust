use brainrust::{Program,BFProgram};

fn main() {
  let expression = std::env::args().nth(1).expect("no expression given");
  
  let mut program: Program = BFProgram::new();

  for instruction in expression.chars() {
    match instruction {
      '.' => program.print(),
      '+' => program.plus(),
      '>' => program.shift_right(),
      '<' => program.shift_left(),
      _ => ()
    }
  }
}
