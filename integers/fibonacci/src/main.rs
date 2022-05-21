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
  fn take_first(n: usize) -> Vec<usize> {
    let fib = Fib {x: (1,0)};
    fib.take(n).collect()
  }
}

fn main () {
    println!("{:?}", Fib::take_first(10));
}
  