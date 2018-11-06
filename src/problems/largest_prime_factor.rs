#![allow(dead_code)]
pub fn largest() -> i64 {
  let mut n = 600_851_475_143;
  let mut factors = Vec::new();

  for d in 2.. {
    while n % d == 0 {
      n = n / d;
      factors.push(d);
    }
    if d * d > n {
      if n != 1 {
        factors.push(n);
      }
      break;
    }
  }

  factors.into_iter().max().unwrap()
}
