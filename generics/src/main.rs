fn main() {
  let number_list: Vec<i32> = vec![12,42];
  let largest_number: i32 = get_largest(number_list);
  println!("Larges Number is {}", largest_number);

  let char_list: Vec<char> = vec!['a','b'];
  let largest_character: char = get_largest(char_list);
  println!("Larges Character is {}", largest_character);

  let p1: Point<i32> = Point {x: 5, y: 10};
  println!("Point is {}", p1.x);

  let p2: Point<f64> = Point {x: 5.0, y: 10.0};
  println!("Point is {}", p2.y);

  let p3: AdvancedPoint<i32, i32> = AdvancedPoint {x: 1, y: 2};
  let p4: AdvancedPoint<char, char> = AdvancedPoint {x: '4', y: '6'};
  let p5: AdvancedPoint <i32, char> = p3.mix_up(p4);
  println!("AdvancedPoint is {}, and {}", p5.x, p5.y);
}

struct Point<T> {
  x: T,
  y: T
}

/* Defining a Generic Method on Struct `Point` */
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

/* Defining a Method on Struct `Point` with Type of `f64` */
impl Point<f64> {
  fn y(&self) -> f64 {
    self.y
  }
}

struct AdvancedPoint<T, U> {
  x: T,
  y: U
}

impl<T, U> AdvancedPoint<T, U> {
  /* Generics `V` and `W` are scoped to the Scope of the Function `mix_up` */
  fn mix_up<V, W>(self, other: AdvancedPoint<V, W>) -> AdvancedPoint<T, W> {
    AdvancedPoint {
      x: self.x,
      y: other.y
    }
  }
}

/* Using Traits `PartialOrd` and `Copy` to make Generic Type comparable */
/* Trait `PartialOrd` means that Generic Type can be ordered - For Example Primitive Types like Integer and Characters */
/* Trait `Copy` means that Generic Type can be copied */
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
  let mut largest: T = list[0];
  for el in list {
    if el > largest {
      largest = el;
    }
  }
  largest
}