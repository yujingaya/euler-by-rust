struct Prime {
  cache: Vec<i64>
}

impl Prime {
  fn new() -> Prime {
    Prime { cache: vec!(2) }
  }

  fn get(&self) -> i64 {
    *self.cache.last().unwrap()
  }
}

impl Iterator for Prime {
  type Item = i64;

  fn next(&mut self) -> Option<i64> {
    let p = self.get();

    let next = ((p + 1)..).find(|&n|
      self.cache.iter().all(|&d| n % d != 0)
    ).unwrap();

    self.cache.push(next);

    Some(p)
  }
}

pub fn largest() -> i64 {
  let mut n = 600_851_475_143;
  let p = Prime::new();
  let mut factors = Vec::new();

  for k in p {
    while n % k == 0 {
      n = n / k;
      factors.push(k);
    }
    if k * k > n {
      if n != 1 {
        factors.push(n);
      }
      break;
    }
  }
  
  *factors.iter().max().unwrap()
}
