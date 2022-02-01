fn get_nth_arguments(number: usize) -> String {
    // return std::env::args().nth(number).unwrap();
    // Implicit Return
    std::env::args().nth(number).unwrap()
}

// Deriving Trait `Debug` for `Arguments`
#[derive(Debug)]
pub struct Arguments {
    // `pub` makes the Field accessible outside the current Module
    pub first_image: String,
    pub second_image: String,
    pub result_image: String,
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            // Argument at Index 0 is Path to Binary
            // At the following Indexes are the Arguments passed through the Terminal
            first_image: get_nth_arguments(1),
            second_image: get_nth_arguments(2),
            result_image: get_nth_arguments(3),
        }
    }
}