use std::fs::File;
use std::io::ErrorKind;

fn main() {
  /* The Operator `?` opens the File and returns it otherwise the Function will end and return the Error */
let _f = File::open("f.txt");

/* Using Variable Shadowing */
let _f: File = match _f {
  Ok(file) => file,
  Err(error) => match error.kind() {
    /* Creating a new File can also fail therefore another `match` is necessary */
    ErrorKind::NotFound => match File::create("f.txt") {
      Ok(f_created) => f_created,
      /* Using Macro `panic` to stop the Program immediately if reached these Macro */
      Err(e_created) => panic!("Problem creating File: {:?}", e_created)
    }
    any_error => {
      panic!("Catching any Problem by Opening the File: {:?}", any_error)
    }
  }
};

/* Same Error Handling using Closures (Anonymous Function) */
/* Using Variable Shadowing */
let _f: File = File::open("f.txt").unwrap_or_else(|error| {
  if error.kind() == ErrorKind::NotFound {
    File::create("f.txt").unwrap_or_else(|error| {
      panic!("Problem creating File: {:?}", error);
    })
  } else {
    panic!("Catching any Problem by Opening the File: {:?}", error);
  }
});
}