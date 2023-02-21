pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn iterator_demonstration() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // The Method `next` needs a mutable Iterator to change the inner State of the Iteration
    // The Iterator returns immutable References
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1_it = v1.iter();
    let sum: i32 = v1_it.sum();
    assert_eq!(sum, 6);
}

fn main() {
  let v1: Vec<i32> = vec![1, 2, 3];
  // Iterators are lazy therefore if the Iterators is not used it wil be not created
  let v1_iter = v1.iter();
  for value in v1_iter {
    println!("Element: {}", value);
  }

  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  assert_eq!(v2, vec![2, 3, 4]);
}

struct Counter {
  count: u32
}

impl Counter {
  fn new() -> Counter {
    Counter {
      count: 0
    }
  }
}

// Iterator Trait for Counter
impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self. count < 4 {
      self.count +=1;
      Some(self.count)
    } else {
      None
    }
  }
}

#[test]
fn counter_demonstration() {
  let mut counter: Counter = Counter::new();
  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), None);

  // let sum: u32 = Counter::new()
  //   .zip(Counter::new().skip(2))
  //   .map(|(a, b) | a * b)
  //   .filter(|x| x % 3 == 0)
  //   .sum();
  // assert_eq!(sum, 12);
}