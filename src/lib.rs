pub trait BFProgram {
  fn new() -> Self;
  fn plus(&mut self) -> ();
  fn minus(&mut self) -> ();
  fn shift_left(&mut self) -> ();
  fn shift_right(&mut self) -> ();
  fn left_bracket(&mut self) -> ();
  fn right_bracket(&mut self) -> ();
  fn print(&self) -> ();
}

#[derive(Debug, Clone)]
pub struct Program {
  pub tape: Vec<u8>,
  pub code: Vec<char>,
  pub position: usize,
  pub pointer: usize,
  pub bracket_stack: Vec<usize>
}

impl BFProgram for Program {

  fn new() -> Program {
    Program { 
      tape: vec![0],
      code: vec![],
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

  fn left_bracket(self: &mut Program) -> () {
    let mut openBrackets = 1;
    if self.tape[self.position] > 0 {
      self.bracket_stack.push(self.pointer);
    } else {
      while openBrackets > 0 {
        if self.code[self.pointer] == ']' {
          openBrackets -= 1;
        } else if self.code[self.pointer] == '[' {
          openBrackets += 1;
        }
      }
    }
  }

  fn right_bracket(self: &mut Program) -> () {
    let final_length = self.bracket_stack.len() - 1;
    self.pointer = self.bracket_stack.remove(final_length) - 1;
  }

  fn print(self: &Program) -> () {
    println!("{}", self.tape[self.position] as char
    )
  }
}
