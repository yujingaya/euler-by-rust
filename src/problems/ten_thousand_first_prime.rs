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
      self.cache.iter()
        .take_while(|&d| d * d <= n)
        .all(|&d| n % d != 0)
    ).unwrap();

    self.cache.push(next);

    Some(p)
  }
}

pub fn get() -> i64 {
  let p = Prime::new();

  p.skip(10000).next().unwrap()
}
