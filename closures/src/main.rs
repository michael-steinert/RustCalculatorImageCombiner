use std::thread;
use std::time::Duration;

fn expensive_calculation(number: u32) -> u32 {
  println!("Calculating..");
  thread::sleep(Duration::from_secs(2));
  number
}

fn main() {
  let expensive_number: u32 = expensive_calculation(42);
  /* Variable stores the Close and not the Return Value */
  let expensive_closure = |number: u32| -> u32 {
    println!("Calculating..");
    thread::sleep(Duration::from_secs(2));
    number
  };
  println!("Number from Function {} and from Closure {}", expensive_number, expensive_closure(21));

  let output_closure = |x: String| x;
  let str: String = output_closure(String::from("Michael"));

  let mut expensive_closure_with_cache = Cacher::new(|number: u32| -> u32 {
    println!("Calculating..");
    thread::sleep(Duration::from_secs(2));
    number
  });
  println!("Number from Closure with Cache {}", expensive_closure_with_cache.value(42));

  let x: i32 = 2;
  /* Closures can dynamically capture the Environment Variables from outside their Scope */
  /* Storing dynamic Environment Variables requires additional Memory */
  let equal_to_x = |z: i32| z == x;
  let y: i32 = 2;
  assert!(equal_to_x(y));
}

struct Cacher<T: Fn(u32) -> u32> {
  calculation: T,
  value: Option<u32>
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32
{
  /* */
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    /* Checking the optional Type `value` */
    match self.value {
      Some(val) => val,
      None => {
        /* Calling the Closure `calculation` and passing Variable `arg` */
        /* Mutate the `Cacher` to cache the Value */
        let val: u32 = (self.calculation)(arg);
        self.value = Some(val);
        val
      }
    }
  }
}