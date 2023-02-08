#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn add_two(a: i32) -> i32 {
  a + 2
}

fn greeting(name: &str) -> String {
  format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
  /* Import all from `Rectangle` Module into Test */
  use super::*;

  #[test]
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("Equation is false"))
    }
  }

  #[test]
  fn larger_can_hold_smaller_rectangle() {
    let larger_rectangle: Rectangle = Rectangle {
      width: 4,
      height: 6
    };
    let smaller_rectangle: Rectangle = Rectangle {
      width: 2,
      height: 3
    };
    assert!(larger_rectangle.can_hold(&smaller_rectangle));
  }

  #[test]
  fn smaller_can_not_hold_larger_rectangle() {
    let larger_rectangle: Rectangle = Rectangle {
      width: 4,
      height: 6
    };
    let smaller_rectangle: Rectangle = Rectangle {
      width: 2,
      height: 3
    };
    assert!(!smaller_rectangle.can_hold(&larger_rectangle));
  }

  #[test]
  fn it_adds_two() {
    assert_eq!(add_two(1), 3);
  }

  #[test]
  fn greeting_does_contains_name() {
    let greeting: String = greeting("Michael");
    assert!(
      greeting.contains("Michael"),
      "Greeting does not contain Name - Value was {}",
      greeting
    );
  }

  #[test]
  #[should_panic(expected = "Greeting does not contain Name - Value was Hello Bruno")]
  fn greeting_does_not_contain_name() {
    let greeting: String = greeting("Bruno");
    assert!(
      greeting.contains("Michael"),
      "Greeting does not contain Name - Value was {}",
      greeting
    );
  }
}