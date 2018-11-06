#![allow(dead_code)]
struct Fibonacci {
  curr: i32,
  next: i32
}

impl Fibonacci {
  fn new() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
  }
}

impl Iterator for Fibonacci {
  type Item = i32;

  fn next(&mut self) -> Option<i32> {
    let next = self.curr + self.next;

    self.curr = self.next;
    self.next = next;

    Some(self.curr)
  }
}

pub fn even() -> i32 {
  let f = Fibonacci::new();

  f.filter(|x| x % 2 == 0)
    .take_while(|&x| x < 4_000_000)
    .sum()
}
