struct Fib {
    x: (usize, usize)
  }
  
impl Iterator for Fib {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
    self.x = (self.x.1, self.x.0 + self.x.1);
    Some(self.x.0)
  }
}

impl Fib {
  fn new() -> Fib {
    Fib { x: (1, 0) }
  }

  fn take_first(n: usize) -> Vec<usize> {
    Fib::new().take(n).collect()
  }
}

fn main () {
  let n: usize = std::env::args().nth(1).unwrap().parse().unwrap();
  println!("{:?}", Fib::take_first(n));
}
  