/* Compile Directive that allow unused Variables */
#![allow(unused_variables)]

fn main() {
    /* Running Macro `println` */
    println!("Rust Essentials");
    let first_number: i32 = 42;
    let second_number: i32 = 21;
    println!("First Number is {}, Second is {}", first_number, second_number);
    println!("Second Number is {1}, First is {0}", first_number, second_number);
    println!("Result is {result}", result = first_number + second_number);
    /* Printing Data Structure like Array or Tuples */
    println!("Tuple is {:?}", (42, 21));

    /* Mutable Variables */
    let mut x: i32 = 5;
    x = 6;

    /* Shadow Variable - old Variable is redefined */
    let y: i32 = 5;
    let y = y + 1;
    let abc: &str = "abc";
    let abc = abc.len();

    /* Constant cannot be redefined */
    const ONE: u32 = 1;

    /* Types */
    let mut hello = String::from("Hello");

    let tuple: (i32, f64, u8) = (42, 4.2, 21);
    let (a, b, c) = tuple;
    let is_equal: bool = a == tuple.0;
    println!("Tuple is {:?}", is_equal);

    let a: [i32; 3] = [1, 2, 3];
    let first: i32 = a[0];
    let b: [i32; 3] = [0; 3];

    /* Loops */
    for element in a.iter() {
        println!("Value is: {}", element);
    }

    for number in 1..4 {
        /* Range is 1 to 3*/
        println!("Number is: {}", number);
    }

    /* Expressions */
    let expression: i32 = {
        let mut x: i32 = 3;
        x = x * 2;
        /* Statement - implicit Return */
        x
    };
    println!("Expression is {:?}", expression);

    /* Strings */
    let mut s1 = String::from("hello");
    /* `s1` and `s2` are seperated in Memory (Copies) because there can be only one Ownership */
    let s2 = s1.clone();
    /* Using Reference instead of Ownership - Immutable Ownership is shared over `s1` and `s3` */
    let s3 = &s1;
    /* Using Reference instead of Ownership - Mutable Ownership is borrowed to `s3` */
    /* While `s4` is borrowing the Variable `s1` can not change it */
    let s4 = &mut s1;
    println!("String1 is {:?} and String2 is {:?}", s1, s2);

    /* Functions */
    let mut number: i32 = 21;
    plus_one_by_reference(&mut number);
    println!("Number has changed to {:?}", number);
}

fn plus_one_by_reference(number: &mut i32) {
    *number = *number + 42;
}
