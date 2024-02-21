mod second_mod;
mod first_mod;
mod new_mod;
mod front_of_house;

pub use crate::front_of_house::hosting;
pub use crate::new_mod::mods;
pub use crate::second_mod::second_second_mod as as_second_second_mod;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    second_mod::print_mod();
    as_second_second_mod::method_in_second_second_mod();
}

pub fn print_mod() {
    mods::print_mod();
}

pub fn print_mod2() {
    mods::print_mod();
}

pub fn method_in_first_mod() {
    first_mod::method_in_first_mod();
}

pub fn method_in_second_mod() {
    second_mod::print_mod();
}

// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
