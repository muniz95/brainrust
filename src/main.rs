use brainrust::{BFProgram};

fn main() {
  let program = BFProgram {
    tape: vec![0],
    position: 0
  };

  program.plus();
  program.minus();
  program.shift_left();
  program.shift_right();
}
