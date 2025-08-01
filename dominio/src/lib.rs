
pub mod errors;
pub mod value_objects;
pub mod entities;
pub mod aggregates;
pub mod services;
pub mod specifications;
pub mod policies;
pub mod events;
pub mod repositories;
pub mod contracts;

pub mod prelude;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
