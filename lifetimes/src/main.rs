use std::fmt::Display;

struct User<'b> {
  name: &'b str
}

impl<'b> User<'b> {
  fn return_name(&'b self, greeting: &str) -> &'b str {
    println!("{} {}", greeting, self.name);
    self.name
  }
}

fn main() {
  let str1: String = String::from("Michael");
  {
    let str2: String = String::from("Bruno"); 
    let longest_name: &str = get_longest_name(str1.as_str(), str2.as_str());
    println!("Longest String is {}", longest_name);

    let longest_name_with_greeting: &str = get_longest_name_with_greeting(str1.as_str(), str2.as_str(), "Hey");
    println!("Longest Name with Greeting is {}", longest_name_with_greeting);
  }
  // Does not work because the Return Type has the same Lifetime as the Parameter with the smallest Lifetime
  // str2 in the inner Scope has shorter Lifetime than str1 in the outer therefore the Return Type is here not valid
  // println!("Longest String is {}", longest_name);
  {
    /* Struct is only valid if the used Variable `str1` is in its Scope */
    let user: User = User {
      name: str1.as_str()
    };
    println!("User is {}", user.name);
    user.return_name("Hello");
  }
}

/* Using a Generic Lifetime Annotation to keep the Variables over the whole Execution Block */
/* Convention for Generic Lifetime Annotation: should start with an Apostrophe and Letter  */
/*
&i32 - a Reference
&'a i32 - a Reference with an explicit Lifetime
&'a mut i32 - a mutable Reference with an explicit Lifetime
*/
/* The Return Type will have the smallest Lifetime of the given Parameter */
fn get_longest_name<'a>(a: &'a str, b: &'a str) -> &'a str {
  if a.len() > b.len() {
    a
  } else {
    b
  }
}

fn get_longest_name_with_greeting<'c, T>(
  a: &'c str,
  b: &'c str,
  greeting: T
) -> &'c str
where
  T:Display
{
  if a.len() > b.len() {
    println!("{} {}", greeting, a);
    a
  } else {
     println!("{} {}", greeting, b);
    b
  }
}  
