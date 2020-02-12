pub trait BFProgram {
  fn new() -> Self;
  fn plus(&mut self) -> ();
  fn minus(&mut self) -> ();
  fn shift_left(&mut self) -> ();
  fn shift_right(&mut self) -> ();
  fn print(&self) -> ();
}

#[derive(Debug, Clone)]
pub struct Program {
  pub tape: Vec<u8>,
  pub position: usize,
  pub pointer: usize,
  pub bracket_stack: Vec<i32>
}

impl BFProgram for Program {

  fn new() -> Program {
    Program { 
      tape: vec![0],
      position: 0,
      bracket_stack: vec![0],
      pointer: 0
    }
  }

  fn plus(self: &mut Program) -> () {
    self.tape[self.position] += 1;
  }

  fn minus(self: &mut Program) -> () {
    self.tape[self.position as usize] -= 1;
  }
  
  fn shift_left(self: &mut Program) -> () {
    self.position -= 1;
  }

  fn shift_right(self: &mut Program) -> () {
    self.position += 1;
    if self.tape.len() == self.position {
      self.tape.push(0);
    }
  }

  fn print(self: &Program) -> () {
    println!("{}", self.tape[self.position] as char
    )
  }
}
