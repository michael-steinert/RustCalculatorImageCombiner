fn main() {
    /* Strings are stored on the Heap */
    let s1 = String::from("Bruno");
    /* Ownership is moved from `s1` to `s2` - `s1` can no longer access the Variable */
    let s2 = s1;
    /* A new String is created as a Copy from `s2` */
    let s3 = s2.clone();

    println!("Hello {}", s3);

    /* Ownership of `s3` is no longer available in these Scope only in Scope of `other_scope` */
    /* The Variable `s3` is dropped as soon as the Scope of `other_scope` has been completed */
    other_scope_heap(s3);

    let x = 42;
    other_scope_stack(x);
    /* Variable `x` is still available because a Copy was created for the other Scope `other_scope_stack` */
    println!("{}", x);

    /* Using Reference of `s2` so after the other Scope `calculate_length` has been completed the Variable `s2` is still available */
    /* Borrowing Variable but do not take the Ownership with References */
    let len_of_s2 = calculate_length(&s2);
    println!("Length of s2 is {}", len_of_s2);
    println!("{}", s2);

    let mut s4 = s2;
    concatenate_string(&mut s4);

    /* A Reference can be used either as one mutable Reference or as any Number of immutable References */
    let mut s5 = String::from("Buddy");
    let r1 = &s5;
    let r2 = &s5;
    /* Scope of `r1` and `r2` ends with the Macro `println!()` */
    println!("{}, {}", r1, r2);
    let r3 = &mut s5;
    println!("{}", r3);

    /* Slices do not take Ownership of the underlying Data */
    let s6 = String::from("Bruno Buddy");
    let first_word = get_first_word(&s6);
    println!("First Word is {}", first_word);
}

fn other_scope_heap(s: String) {
    println!("Hey {}", s);
}

fn other_scope_stack(i: i32) {
    println!("{}", i);
}

fn calculate_length(s: &String) -> usize {
    /* References are immutable */
    let length = s.len();
    /* Implicit Return */
    length
}

fn concatenate_string(s: &mut String) {
    s.push_str("!");
    println!("{}", s);
}

fn get_first_word(s: &str) -> &str {
    let string_bytes = s.as_bytes();
    for (i, &item) in string_bytes.iter().enumerate() {
        /* A Blank Space indicates the End of a Word */
        if item == b' ' {
            /* String Slice from Begin to Blank Space of Vector */
            return &s[0..i];
        }
    }
    /* Entire String is one Word - String Slice from Begin to End of Vector */
    &s[..]
}