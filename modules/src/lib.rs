use std::fmt::Result;
/* Renaming so there are not Conflicts in Naming in the Scope */
use std::io::Result as IoResult;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_list() {

        }
        pub fn add_to_list_with_result() -> Result {}
        pub fn add_to_list_with_io_result() -> IoResult {}
    }
}

/* Bringing Module into Scope to shorten the Path */
use crate::front_of_house::hosting;
/* `self` is referencing the current Module */
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    /* Absolute Path - from basic Module `crate` */
    crate::front_of_house::hosting::add_to_list();

    /* Relative Path - from current Module */
    front_of_house::hosting::add_to_list();

    /* Path is using the Scope */
    hosting::add_to_list();
}

/* Content of Module is split into  Files */
mod back_of_house;

pub fn pay_at_restaurant() {
    /* Absolute Path - from basic Module `crate` */
    crate::back_of_house::managing::pay_list();
}
