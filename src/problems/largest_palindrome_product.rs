fn is_palindrome(n: i32) -> bool {
  let n = n.to_string();
  let n = n.as_bytes();
  let len = n.len();

  (0..len / 2).all(|i|
    n[i] == n[len - i - 1]
  )
}

pub fn largest() -> i32 {
  let mut max = 0;
  for i in 100..1000 {
    for j in 100..1000 {
      let product = i * j;
      if product > max && is_palindrome(product) {
        max = product
      }
    }
  }

  max
}
